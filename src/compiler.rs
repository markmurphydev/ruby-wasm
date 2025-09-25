//! Compiles a Ruby AST to a Wasm module

// W for Wasm
use crate::node::Subsequent;
use crate::wasm::module::Module;
use crate::wasm::types::{GlobalType, Mutability, NumberType, ValType, UNITYPE};
use crate::wasm::UnaryOp;
// R for Ruby
use crate::{node as R, FunctionBuilder, InstrSeqBuilder};
use crate::{runtime, ArenaProvider};

pub const RUBY_TOP_LEVEL_FUNCTION_NAME: &str = "__ruby_top_level_function";

/// We give fixnums half an i31, marking MSB 1
/// (0b1xx_xxxx...): i31
const FIXNUM_BIT_WIDTH: u32 = 30;

/// Fixnums are identified with a 1 in the MSB of the i31
pub const FIXNUM_MARKER: i32 = 1 << 30;

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
        true,
        Box::new([]),
        Box::new([UNITYPE.into_result_type()]),
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

fn compile_statements<A: ArenaProvider>(
    ctx: &mut CompileCtx<'_>,
    builder: &mut InstrSeqBuilder<'_, A>,
    statements: &R::Statements,
) {
    let R::Statements { body } = statements;

    // In Ruby, every expression returns a value or nil.
    // Suppress all values except the last.
    let last_statement_idx = body.len() - 1;
    for expr in (&body[0..last_statement_idx]).into_iter() {
        compile_expr(ctx, builder, expr);
        builder.drop();
    }

    compile_expr(ctx, builder, &body[last_statement_idx])
}

fn compile_expr<A: ArenaProvider>(
    ctx: &mut CompileCtx<'_>,
    builder: &mut InstrSeqBuilder<A>,
    expr: &R::Expr,
) {
    match expr {
        &R::Expr::Integer(n) => compile_integer(ctx, builder, n),
        R::Expr::False => const_i31(builder, 0b001),
        R::Expr::True => const_i31(builder, 0b0011),
        R::Expr::Nil => const_i31(builder, 0b0101),
        R::Expr::If(if_expr) => compile_if_expr(ctx, builder, &*if_expr),
        R::Expr::While(while_expr) => compile_while_expr(ctx, builder, &*while_expr),
        R::Expr::Until(until_expr) => compile_until_expr(ctx, builder, &*until_expr),
    }
}

/// Convert the given integer into a Wasm fixnum or const global representation
fn compile_integer<A: ArenaProvider>(
    ctx: &mut CompileCtx<'_>,
    builder: &mut InstrSeqBuilder<A>,
    n: i64,
) {
    // Strategy:
    // Determine whether we're in range of a fixnum
    // If not, add a const global int and get it
    // TODO -- do this _somewhere else_

    /// Minimum size required for 2's complement representation of the given number
    /// Strategy from:
    /// https://internals.rust-lang.org/t/add-methods-that-return-the-number-of-bits-necessary-to-represent-an-integer-in-binary-to-the-standard-library/21870/7
    fn bit_width(n: i64) -> u32 {
        i64::BITS - n.abs().leading_zeros() + 1
    }

    // If if-let guards were stable I'd use those with try_into.
    match n {
        n if bit_width(n) <= FIXNUM_BIT_WIDTH => {
            let fixnum = FIXNUM_MARKER | i32::try_from(n).unwrap();
            const_i31(builder, fixnum);
        }
        n if bit_width(n) <= i64::BITS => {
            let n = i64::try_from(n).unwrap();
            // TODO -- Need to intern this.
            //  If you have 2 of the same-valued global it probably breaks at validation time.
            //  Also, does this introduce illegal identifier characters?
            let global_id = format!("$global-i32-{}", n);
            let ty = GlobalType {
                mutability: Mutability::Const,
                value_type: ValType::NumberType(NumberType::I64),
            };

            ctx.module.globals.add(global_id.clone(), ty, |builder| {
                builder.i64_const(n);
            });

            builder.global_get(global_id);
        }
        _ => {
            todo!(
                "Bignums not yet implemented.
                  [n={:x}] larger than W::I64",
                22
            )
        }
    }
}

fn compile_if_expr<A: ArenaProvider>(
    ctx: &mut CompileCtx<'_>,
    builder: &mut InstrSeqBuilder<A>,
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
            Subsequent::None => const_i31(else_builder, 0b0101),
            Subsequent::Elsif(if_expr) => compile_if_expr(ctx, else_builder, &if_expr),
            Subsequent::Else(else_expr) => {
                compile_statements(ctx, else_builder, &else_expr.statements)
            }
        },
    );
}

fn compile_while_expr<A: ArenaProvider>(
    ctx: &mut CompileCtx<'_>,
    builder: &mut InstrSeqBuilder<A>,
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

    builder.loop_(label.clone(), |builder| {
        builder.if_else(
            ctx,
            |ctx, builder| {
                compile_expr_to_wasm_predicate(ctx, builder, predicate);
            },
            |ctx, builder| compile_statements(ctx, builder, statements),
            |_ctx, builder| {
                builder.br(label);
            },
        );
    });
}

fn compile_until_expr<A: ArenaProvider>(
    ctx: &mut CompileCtx<'_>,
    builder: &mut InstrSeqBuilder<A>,
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

    builder.loop_(label.clone(), |builder| {
        builder.if_else(
            ctx,
            |ctx, builder| {
                compile_expr_to_wasm_predicate(ctx, builder, predicate);
                // `binary_not ≡ eqz` when result is interpreted as boolean
                builder.unop(UnaryOp::I32Eqz);
            },
            |ctx, builder| compile_statements(ctx, builder, statements),
            |_ctx, builder| {
                // Just return nil
                // const_i31(builder, 0b0101)
                builder.br(label);
            },
        );
    });
}

/// Turns a Ruby Expr into a Wasm predicate.
/// A ruby Expr evaluates to a ruby-value (True, False, Nil, ...)
/// To use as a Wasm predicate, we need to test whether the result is truthy or not.
/// TODO -- right now we pretend that "truthy" is "not-false"
fn compile_expr_to_wasm_predicate<A: ArenaProvider>(
    ctx: &mut CompileCtx<'_>,
    builder: &mut InstrSeqBuilder<A>,
    expr: &R::Expr,
) {
    compile_expr(ctx, builder, expr);
    runtime::is_false(builder);
    builder.unop(UnaryOp::I32Eqz);
}

fn const_i31<A: ArenaProvider>(builder: &mut InstrSeqBuilder<A>, val: i32) {
    builder.i32_const(val).unop(UnaryOp::RefI31);
}
