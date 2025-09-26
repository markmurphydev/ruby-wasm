//! Compiles a Ruby AST to a Wasm module

// W for Wasm
use crate::node::{GlobalVariableRead, GlobalVariableWrite, Subsequent};
use crate::wasm::UnaryOp;
use crate::wasm::module::{GlobalBuilder, Module};
use crate::wasm::types::{GlobalType, Mutability, NumType, ValType};
// R for Ruby
use crate::unitype::Unitype;
use crate::{runtime};
use crate::{FunctionBuilder, InstrSeqBuilder, node as R};
use crate::wasm::function::ExportStatus;

pub const RUBY_TOP_LEVEL_FUNCTION_NAME: &str = "__ruby_top_level_function";

pub struct CompileCtx<'a> {
    pub module: &'a mut Module,
}

pub fn compile(program: &R::Program) -> Module {
    let mut module = Module::new();
    let mut ctx = CompileCtx {
        module: &mut module,
    };

    // Build the top-level function
    let mut top_level_builder = FunctionBuilder::new(
        RUBY_TOP_LEVEL_FUNCTION_NAME,
        ExportStatus::Exported,
        Box::new([]),
        Box::new([Unitype::UNITYPE.into_result_type()]),
    );
    compile_program(&mut ctx, &mut top_level_builder, program);
    top_level_builder.finish(&mut module.funcs);

    module
}

fn compile_program(
    ctx: &mut CompileCtx<'_>,
    top_level_builder: &mut FunctionBuilder,
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
    // Suppress all values except the last.
    let last_statement_idx = body.len() - 1;
    for expr in (&body[0..last_statement_idx]).into_iter() {
        compile_expr(ctx, builder, expr);
        builder.drop(ctx);
    }

    compile_expr(ctx, builder, &body[last_statement_idx])
}

fn compile_expr(
    ctx: &mut CompileCtx<'_>,
    builder: &InstrSeqBuilder,
    expr: &R::Expr,
) {
    match expr {
        &R::Expr::Integer(n) => compile_integer(ctx, builder, n),
        R::Expr::False => const_i31(ctx, builder, Unitype::FALSE_BIT_PATTERN),
        R::Expr::True => const_i31(ctx, builder, Unitype::TRUE_BIT_PATTERN),
        R::Expr::Nil => const_i31(ctx, builder, Unitype::NIL_BIT_PATTERN),

        R::Expr::GlobalVariableWrite(global_write) => compile_global_variable_write(ctx, global_write),
        R::Expr::GlobalVariableRead(global_read) => compile_global_variable_read(ctx, builder, global_read),

        R::Expr::If(if_expr) => compile_if_expr(ctx, builder, &*if_expr),
        R::Expr::While(while_expr) => compile_while_expr(ctx, builder, &*while_expr),
        R::Expr::Until(until_expr) => compile_until_expr(ctx, builder, &*until_expr),
    }
}

/// Convert the given integer into a Wasm fixnum or const global representation
fn compile_integer(
    ctx: &mut CompileCtx<'_>,
    builder: &InstrSeqBuilder,
    n: i64,
) {
    let unitype = Unitype::from_integer(n);
    match unitype {
        fixnum @ Unitype::Fixnum(_) => const_i31(ctx, builder, fixnum.to_i31_bits()),
        Unitype::HeapNum(heapnum) => {
            // `heapnum` is a constant value.
            // So, create a global and get its value.

            // TODO -- We need to dedup constant heapnums.
            //  If you have 2 of the same-valued global this probably breaks at validation time.
            let global_id = format!("$global-i32-{}", heapnum);

            let global_builder = GlobalBuilder::new(ctx.module, global_id.clone());
            global_builder.instr_seq().i64_const(ctx, heapnum);
            global_builder.finish(&mut ctx.module.globals);

            builder.global_get(ctx, global_id);
        }
        _ => unreachable!(),
    }
}

/// Add a global to the Module, setting its value to the write's rhs.
fn compile_global_variable_write(
    ctx: &mut CompileCtx<'_>,
    global_write: &GlobalVariableWrite,
) {
    let GlobalVariableWrite { name, expr } = global_write;

    let global_builder = GlobalBuilder::new(ctx.module, name.clone());
    let instr_seq_builder = global_builder.instr_seq();
    compile_expr(ctx, &instr_seq_builder, expr);
    global_builder.finish(&mut ctx.module.globals);
}

fn compile_global_variable_read(
    ctx: &mut CompileCtx<'_>,
    builder: &InstrSeqBuilder,
    global_read: &GlobalVariableRead,
) {
    let GlobalVariableRead { name } = global_read;

    builder.global_get(ctx, name.clone());
}

fn compile_if_expr(
    ctx: &mut CompileCtx<'_>,
    builder: &InstrSeqBuilder,
    if_expr: &R::If,
) {
    let R::If {
        predicate,
        statements,
        subsequent,
    } = if_expr;
    builder.if_else(
        ctx,
        |ctx, pred_builder| compile_expr_to_wasm_predicate(ctx, pred_builder, predicate),
        |ctx, then_builder| {
            compile_statements(ctx, then_builder, statements);
        },
        |ctx, else_builder| match subsequent {
            Subsequent::None => const_i31(ctx, else_builder, Unitype::NIL_BIT_PATTERN),
            Subsequent::Elsif(if_expr) => compile_if_expr(ctx, else_builder, &if_expr),
            Subsequent::Else(else_expr) => {
                compile_statements(ctx, else_builder, &else_expr.statements)
            }
        },
    );
}

fn compile_while_expr(
    ctx: &mut CompileCtx<'_>,
    builder: &InstrSeqBuilder,
    while_expr: &R::While,
) {
    // while ->
    // (loop
    //   (if UNITYPE predicate
    //      (then statements)
    //      (else (break)))
    let R::While {
        predicate,
        statements,
    } = while_expr;

    let label = "$while".to_string();

    builder.loop_(ctx, label.clone(), |ctx, builder| {
        builder.if_else(
            ctx,
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

fn compile_until_expr(
    ctx: &mut CompileCtx<'_>,
    builder: &InstrSeqBuilder,
    until_expr: &R::Until,
) {
    // TODO -- It might be nicer to have an IR where `until` is lowered to `while`
    // while ->
    // (loop
    //   (if UNITYPE (not predicate)
    //      (then statements)))
    let R::Until {
        predicate,
        statements,
    } = until_expr;
    let label = "$until".to_string();

    builder.loop_(ctx, label.clone(), |ctx, builder| {
        builder.if_else(
            ctx,
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
    runtime::is_false(ctx, builder);
    builder.unop(ctx, UnaryOp::I32Eqz);
}

fn const_i31(ctx: &mut CompileCtx<'_>, builder: &InstrSeqBuilder, val: i32) {
    builder.i32_const(ctx, val).unop(ctx, UnaryOp::RefI31);
}
