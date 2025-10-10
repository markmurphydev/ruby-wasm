//! Compiles a Ruby AST to a Wasm module

// W for Wasm
use crate::unitype::Unitype;
use crate::wasm::UnaryOp;
use crate::wasm::function::ExportStatus;
use crate::wasm::instr_seq::InstrSeqBuilder;
use crate::wasm::module::{GlobalBuilder, Module};
use crate::{FunctionBuilder, node as R};
use std::hash::{DefaultHasher, Hash, Hasher};

pub const RUBY_TOP_LEVEL_FUNCTION_NAME: &str = "__ruby_top_level_function";

pub struct CompileCtx<'a> {
    pub module: &'a mut Module,
}

pub fn compile(ctx: &mut CompileCtx<'_>, program: &R::Program) {
    // Build the top-level function
    let top_level_builder = FunctionBuilder::new(
        ctx,
        RUBY_TOP_LEVEL_FUNCTION_NAME,
        ExportStatus::Exported,
        None,
        Box::new([]),
        Box::new([Unitype::UNITYPE.into_result_type()]),
        vec![]
    );
    compile_program(ctx, &top_level_builder, program);
    top_level_builder.finish(&mut ctx.module.funcs);
}

fn compile_program(
    ctx: &mut CompileCtx<'_>,
    top_level_builder: &FunctionBuilder,
    program: &R::Program,
) {
    compile_statements(ctx, &mut top_level_builder.func_body(), &program.statements);
}

fn compile_statements(
    ctx: &mut CompileCtx<'_>,
    builder: &InstrSeqBuilder,
    statements: &R::Statements,
) {
    let R::Statements { body } = statements;

    // In Ruby, every expression returns a value or nil.
    // If there are no statements, return nil.
    // Suppress all values except the last.
    if body.is_empty() {
        builder.i31_const(ctx, Unitype::NIL_BIT_PATTERN);
    } else {
        let last_statement_idx = body.len() - 1;
        for expr in (&body[0..last_statement_idx]).into_iter() {
            compile_expr(ctx, builder, expr);
            builder.drop(ctx);
        }
        compile_expr(ctx, builder, &body[last_statement_idx]);
    }
}

fn compile_expr(ctx: &mut CompileCtx<'_>, builder: &InstrSeqBuilder, expr: &R::Expr) {
    match expr {
        &R::Expr::Integer(n) => compile_integer(ctx, builder, n),
        R::Expr::SingleQuoteString(s) => compile_single_quote_string(ctx, builder, s),

        R::Expr::False => {
            builder.i31_const(ctx, Unitype::FALSE_BIT_PATTERN);
        }
        R::Expr::True => {
            builder.i31_const(ctx, Unitype::TRUE_BIT_PATTERN);
        }
        R::Expr::Nil => {
            builder.i31_const(ctx, Unitype::NIL_BIT_PATTERN);
        }

        R::Expr::GlobalVariableWrite(global_write) => {
            compile_global_variable_write(ctx, global_write)
        }
        R::Expr::GlobalVariableRead(global_read) => {
            compile_global_variable_read(ctx, builder, global_read)
        }
        R::Expr::ConstantWrite(_constant_write) => todo!(),
        R::Expr::ConstantRead(constant_read_expr) => {
            compile_constant_read_expr(ctx, builder, &*constant_read_expr)
        }

        R::Expr::If(if_expr) => compile_if_expr(ctx, builder, &*if_expr),
        R::Expr::While(while_expr) => compile_while_expr(ctx, builder, &*while_expr),
        R::Expr::Until(until_expr) => compile_until_expr(ctx, builder, &*until_expr),
        R::Expr::Call(call_expr) => compile_call_expr(ctx, builder, &*call_expr),
    }
}

/// Convert the given integer into a Wasm fixnum or const global representation
fn compile_integer(ctx: &mut CompileCtx<'_>, builder: &InstrSeqBuilder, n: i64) {
    let unitype = Unitype::from_integer(n);
    match unitype {
        fixnum @ Unitype::Fixnum(_) => {
            builder.i31_const(ctx, fixnum.to_i31_bits());
        }
        Unitype::HeapNum(heapnum) => {
            // `heapnum` is a constant value.
            // So, create a global and get its value.

            // TODO -- We need to dedup constant heapnums.
            //  If you have 2 of the same-valued global this probably breaks at validation time.
            let global_id = format!("global-i32-{}", heapnum);

            let global_builder =
                GlobalBuilder::new(ctx.module, Unitype::GLOBAL_CONST_TYPE, global_id.clone());
            global_builder.instr_seq().i64_const(ctx, heapnum);
            global_builder.finish(ctx);

            builder.global_get(ctx, global_id);
        }
        _ => unreachable!(),
    }
}

fn compile_single_quote_string(ctx: &mut CompileCtx<'_>, builder: &InstrSeqBuilder, str: &String) {
    // TODO -- Dedup strings.
    let mut hasher = DefaultHasher::new();
    str.hash(&mut hasher);
    let global_id = format!("single-quote-string-{}", hasher.finish());
    let global_builder =
        GlobalBuilder::new(ctx.module, Unitype::GLOBAL_CONST_TYPE, global_id.clone());
    for byte in str.bytes() {
        global_builder.instr_seq().i32_const(ctx, byte as i32);
    }
    let len_bytes = i32::try_from(str.as_bytes().len()).unwrap();
    global_builder
        .instr_seq()
        .array_new_fixed(ctx, "unitype-string".to_string(), len_bytes);
    global_builder.finish(ctx);

    builder.global_get(ctx, global_id.clone());
}

/// Add a global to the Module, setting its value to the write's rhs.
fn compile_global_variable_write(ctx: &mut CompileCtx<'_>, global_write: &R::GlobalVariableWrite) {
    let R::GlobalVariableWrite { name, expr } = global_write;

    let global_builder = GlobalBuilder::new(ctx.module, Unitype::GLOBAL_MUT_TYPE, name.clone());
    let instr_seq_builder = global_builder.instr_seq();
    compile_expr(ctx, &instr_seq_builder, expr);
    global_builder.finish(ctx);
}

fn compile_global_variable_read(
    ctx: &mut CompileCtx<'_>,
    builder: &InstrSeqBuilder,
    global_read: &R::GlobalVariableRead,
) {
    let R::GlobalVariableRead { name } = global_read;

    builder.global_get(ctx, name.clone());
}

fn compile_constant_read_expr(
    ctx: &mut CompileCtx<'_>,
    builder: &InstrSeqBuilder,
    constant_read_expr: &R::ConstantRead,
) {
    // TODO -- Assuming all constants are classes.
    let R::ConstantRead { name } = constant_read_expr;
    builder.global_get(ctx, class_symbolic_identifier(name));
}

fn compile_if_expr(ctx: &mut CompileCtx<'_>, builder: &InstrSeqBuilder, if_expr: &R::If) {
    let R::If {
        predicate,
        statements,
        subsequent,
    } = if_expr;
    builder.if_else(
        ctx,
        Some(Unitype::UNITYPE.into_block_type_result()),
        |ctx, builder| compile_expr_to_wasm_predicate(ctx, builder, predicate),
        |ctx, builder| {
            compile_statements(ctx, builder, statements);
        },
        |ctx, builder| match subsequent {
            R::Subsequent::None => {
                builder.i31_const(ctx, Unitype::NIL_BIT_PATTERN);
            }
            R::Subsequent::Elsif(if_expr) => compile_if_expr(ctx, builder, &if_expr),
            R::Subsequent::Else(else_expr) => {
                compile_statements(ctx, builder, &else_expr.statements)
            }
        },
    );
}

fn compile_while_expr(ctx: &mut CompileCtx<'_>, builder: &InstrSeqBuilder, while_expr: &R::While) {
    // while ->
    // (loop
    //   (if UNITYPE predicate
    //      (then statements)
    //      (else (break)))
    let R::While {
        predicate,
        statements,
    } = while_expr;

    let label = "while".to_string();

    builder.loop_(ctx, label.clone(), |ctx, builder| {
        builder.if_else(
            ctx,
            Some(Unitype::UNITYPE.into_block_type_result()),
            |ctx, builder| {
                compile_expr_to_wasm_predicate(ctx, builder, predicate);
            },
            |ctx, builder| compile_statements(ctx, builder, statements),
            |ctx, builder| {
                builder.br(ctx, label);
            },
        );
    });
}

fn compile_until_expr(ctx: &mut CompileCtx<'_>, builder: &InstrSeqBuilder, until_expr: &R::Until) {
    // TODO -- It might be nicer to have an IR where `until` is lowered to `while`
    // while ->
    // (loop
    //   (if UNITYPE (not predicate)
    //      (then statements)))
    let R::Until {
        predicate,
        statements,
    } = until_expr;
    let label = "until".to_string();

    builder.loop_(ctx, label.clone(), |ctx, builder| {
        builder.if_else(
            ctx,
            Some(Unitype::UNITYPE.into_block_type_result()),
            |ctx, builder| {
                compile_expr_to_wasm_predicate(ctx, builder, predicate);
                // `binary_not ≡ eqz` when result is interpreted as boolean
                builder.unop(ctx, UnaryOp::I32Eqz);
            },
            |ctx, builder| compile_statements(ctx, builder, statements),
            |ctx, builder| {
                builder.br(ctx, label);
            },
        );
    });
}

fn compile_call_expr(ctx: &mut CompileCtx<'_>, builder: &InstrSeqBuilder, call_expr: &R::Call) {
    let R::Call { receiver, name } = call_expr;
    compile_expr(ctx, builder, receiver);
    builder
        .global_get(ctx, string_symbolic_identifier(name))
        .global_get(ctx, "empty-args".to_string())
        .call(ctx, "call".to_string());
}

/// Turns a Ruby Expr into a Wasm predicate.
/// A ruby Expr evaluates to a ruby-value (True, False, Nil, ...)
/// To use as a Wasm predicate, we need to test whether the result is truthy or not.
/// TODO -- right now we pretend that "truthy" is "not-false"
fn compile_expr_to_wasm_predicate(
    ctx: &mut CompileCtx<'_>,
    builder: &InstrSeqBuilder,
    expr: &R::Expr,
) {
    compile_expr(ctx, builder, expr);
    builder
        .call(ctx, "is_false".to_string())
        .unop(ctx, UnaryOp::I32Eqz);
}

fn class_symbolic_identifier(class_name: &str) -> String {
    // Right now, if the whole identifier is lowercase, the lisp reader makes the name uppercase...
    // TODO -- Fix the lisp reader.
    if class_name.chars().all(|c| c.is_lowercase()) {
        let str_upper_case: String = class_name.chars().flat_map(|c| c.to_uppercase()).collect();
        format!("CLASS-{}", str_upper_case)
    } else {
        format!("class-{}", class_name)
    }
}

fn string_symbolic_identifier(str: &str) -> String {
    // Right now, if the whole identifier is lowercase, the lisp reader makes the name uppercase...
    // TODO -- Fix the lisp reader.
    if str.chars().all(|c| c.is_lowercase()) {
        let str_upper_case: String = str.chars().flat_map(|c| c.to_uppercase()).collect();
        format!("STR-{}", str_upper_case)
    } else {
        format!("str-{}", str)
    }
}
