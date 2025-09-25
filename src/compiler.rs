//! Compiles a Ruby AST to a Wasm module
//!
//! NB: Rust signed <-> unsigned casts (using `as`) reinterpret with 2's complement.
//! ```
//! assert_eq!(-1, u64::MAX as i64);
//! ```

use pretty::BuildDoc;
// W for Wasm
use crate::node::Subsequent;
use crate::wasm::UnaryOp;
use crate::wasm::module::{Module, ModuleGlobals};
use crate::wasm::types::{GlobalType, Mutability, NumberType, UNITYPE, ValType};
use crate::{ArenaProvider, runtime, wasm as W};
// R for Ruby
use crate::{FunctionBuilder, InstrSeqBuilder, node as R};

const RUBY_TOP_LEVEL_FUNCTION_NAME: &str = "__ruby_top_level_function";

/// We give fixnums half an i31, marking MSB 1
/// (0b1xx_xxxx...): i31
const FIXNUM_BIT_WIDTH: u32 = 30;

/// Fixnums are identified with a 1 in the MSB of the i31
pub const FIXNUM_MARKER: i32 = 1 << 30;

// pub struct Compiler {
//     // globals: Vec<W::Global>
// }
//
// impl Compiler {
//     pub fn new() -> Self {
//         Self {
//             // globals: vec![]
//         }
//     }
//     // pub fn compile(self, program: R::Program) -> W::Module {
//     //     // Current strategy:
//     //     // We're kinda doing recursive descent into the Wasm module structure.
//     //     // - There is one wasm_module per ruby_program
//     //     // - There is one wasm_function per ruby_program
//     //     //  - `program.statements.body.map(|expr| => expr.to_wasm_instrs())`
//     //
//     //     self.module(program)
//     // }
//     //
//     // fn module(mut self, program: R::Program) -> W::Module {
//     //     let functions = vec![self.function(program.statements)];
//     //
//     //     // Export our top-level function
//     //     let exports = vec![W::FunctionIdx::Id(RUBY_TOP_LEVEL_FUNCTION_NAME.to_string())];
//     //
//     //     W::Module {
//     //         functions,
//     //         exports,
//     //         globals: self.globals,
//     //         start: None,
//     //     }
//     // }
//     //
//     // fn function(&mut self, statements: R::Statements) -> W::Function {
//     //     let body = self.expr(statements);
//     //     W::Function {
//     //         id: Some(RUBY_TOP_LEVEL_FUNCTION_NAME.to_string()),
//     //         body,
//     //     }
//     // }
//     //
//     // /// Compile a Wasm expr.
//     // /// Corresponds to a Ruby `Statements` node.
//     // fn expr(&mut self, statements: R::Statements) -> W::Expr {
//     //     W::Expr(self.ruby_statements(statements))
//     // }
//     //
//     // fn ruby_statements(&mut self, statements: R::Statements) -> Vec<W::Instruction> {
//     //     let mut expr_instrs: Vec<W::Instruction> = vec![];
//     //     for ruby_expr in statements.body {
//     //         let mut instrs = self.ruby_expr(ruby_expr);
//     //         expr_instrs.append(&mut instrs);
//     //     }
//     //     expr_instrs
//     // }
//     //
//     // fn ruby_expr(&mut self, ruby_expr: R::Expr) -> Vec<W::Instruction> {
//     //     match ruby_expr {
//     //         R::Expr::Integer(n) => self.ruby_integer(n),
//     //         R::Expr::True => const_i31(WV::I32::TRUE),
//     //         R::Expr::False => const_i31(WV::I32::FALSE),
//     //         R::Expr::Nil => const_i31(WV::I32::NIL),
//     //         R::Expr::If(if_expr) => vec![W::Instruction::If(self.ruby_if_expr(*if_expr))],
//     //         R::Expr::While(while_expr) => vec![W::Instruction::Loop(self.ruby_while_expr(*while_expr))],
//     //         R::Expr::Until(until_expr) => vec![W::Instruction::Loop(self.ruby_until_expr(*until_expr))],
//     //     }
//     // }
//     //
//     // /// Convert the given integer into a Wasm fixnum or const global representation
//     // fn ruby_integer(&mut self, n: i64) -> Vec<W::Instruction> {
//     //     // Strategy:
//     //     // Determine whether we're in range of a fixnum
//     //     // If not, add a const global int and get it
//     //
//     //     /// Minimum size required for 2's complement representation of the given number
//     //     /// Strategy from:
//     //     /// https://internals.rust-lang.org/t/add-methods-that-return-the-number-of-bits-necessary-to-represent-an-integer-in-binary-to-the-standard-library/21870/7
//     //     fn bit_width(n: i64) -> u32 {
//     //         i64::BITS - n.abs().leading_zeros() + 1
//     //     }
//     //
//     //     // If if-let guards were stable I'd use those with try_into.
//     //     match n {
//     //         n if bit_width(n) <= FIXNUM_BIT_WIDTH => {
//     //             let fixnum = FIXNUM_MARKER | i32::try_from(n).unwrap();
//     //             let fixnum = WV::I32(fixnum as u32);
//     //             vec![W::Instruction::ConstI32(fixnum), W::Instruction::RefI31]
//     //         }
//     //         n if bit_width(n) <= i32::BITS => {
//     //             let n = i32::try_from(n).unwrap();
//     //             // TODO -- Need to intern this.
//     //             //  If you have 2 of the same-valued global it probably breaks at validation time.
//     //             //  Also, does this introduce illegal identifier characters?
//     //             let global_id = format!("$global-i32-{}", n);
//     //             let expr_instrs = vec![W::Instruction::ConstI32(WV::I32(n as u32))];
//     //
//     //             let global = W::Global {
//     //                 id: Some(global_id.clone()),
//     //                 global_type: WT::GlobalType {
//     //                     mutability: WT::Mutability::Const,
//     //                     value_type: WT::ValType::NumberType(WT::NumberType::I32),
//     //                 },
//     //                 expr: W::Expr(expr_instrs)
//     //             };
//     //             self.globals.push(global);
//     //             vec![W::Instruction::GlobalGet(W::GlobalIdx::Id(global_id))]
//     //         }
//     //         // Guard is here for when bignums are implemented.
//     //         n if bit_width(n) <= i64::BITS => {
//     //             let n = i64::try_from(n).unwrap();
//     //             // TODO -- Need to intern this.
//     //             //  If you have 2 of the same-valued global it probably breaks at validation time.
//     //             //  Also, does this introduce illegal identifier characters?
//     //             let global_id = format!("$global-i64-{}", n);
//     //             let expr_instrs = vec![W::Instruction::ConstI64(WV::I64(n as u64))];
//     //
//     //             let global = W::Global {
//     //                 id: Some(global_id.clone()),
//     //                 global_type: WT::GlobalType {
//     //                     mutability: WT::Mutability::Const,
//     //                     value_type: WT::ValType::NumberType(WT::NumberType::I64),
//     //                 },
//     //                 expr: W::Expr(expr_instrs)
//     //             };
//     //             self.globals.push(global);
//     //             vec![W::Instruction::GlobalGet(W::GlobalIdx::Id(global_id))]
//     //         }
//     //         _ => {
//     //             todo!("Bignums not yet implemented.
//     //                   [n={:x}] larger than W::I64",
//     //                   22)
//     //         }
//     //     }
//     // }
//     //
//     // fn ruby_if_expr(&mut self, if_expr: R::If) -> W::If {
//     //     // Mercifully, we can just recurse for the subsequent
//     //     let else_instrs: Vec<W::Instruction> = match if_expr.subsequent {
//     //         R::Subsequent::None => vec![W::Instruction::ConstI32(I32::NIL), W::Instruction::RefI31],
//     //         R::Subsequent::Elsif(if_expr) => vec![W::Instruction::If(self.ruby_if_expr(*if_expr))],
//     //         R::Subsequent::Else(else_expr) => self.ruby_statements(else_expr.statements)
//     //     };
//     //
//     //     W::If {
//     //         label: None,
//     //         block_type: WT::UNITYPE,
//     //         predicate_instrs: self.ruby_expr(if_expr.predicate),
//     //         then_instrs: self.ruby_statements(if_expr.statements),
//     //         else_instrs,
//     //     }
//     // }
//     //
//     // fn ruby_while_expr(&mut self, while_expr: R::While) -> W::Loop {
//     //     // while ->
//     //     // (loop
//     //     //   (if UNITYPE predicate
//     //     //      (then statements)))
//     //
//     //     let predicate_instrs = self.predicate(while_expr.predicate);
//     //
//     //     let else_instrs = vec![W::Instruction::ConstI32(I32::NIL), W::Instruction::RefI31];
//     //
//     //     let loop_inner_if = W::If {
//     //         label: None,
//     //         block_type: WT::UNITYPE,
//     //         predicate_instrs,
//     //         then_instrs: self.ruby_statements(while_expr.statements),
//     //         else_instrs,
//     //     };
//     //     let loop_instrs =  vec![W::Instruction::If(loop_inner_if)];
//     //
//     //     W::Loop {
//     //         label: None,
//     //         block_type: WT::UNITYPE,
//     //         instructions: loop_instrs,
//     //     }
//     // }
//     //
//     // fn ruby_until_expr(&mut self, until_expr: R::Until) -> W::Loop {
//     //     // TODO -- It might be nicer to have an IR where `until` is lowered to `while`
//     //     // while ->
//     //     // (loop
//     //     //   (if UNITYPE (not predicate)
//     //     //      (then statements)))
//     //
//     //     let inverted_predicate = {
//     //         let mut instrs = self.predicate(until_expr.predicate);
//     //         instrs.push(W::Instruction::I32Eqz);
//     //         instrs
//     //     };
//     //
//     //     let else_instrs = vec![W::Instruction::ConstI32(I32::NIL), W::Instruction::RefI31];
//     //
//     //     let loop_inner_if = W::If {
//     //         label: None,
//     //         block_type: WT::UNITYPE,
//     //         predicate_instrs: inverted_predicate,
//     //         then_instrs: self.ruby_statements(until_expr.statements),
//     //         else_instrs,
//     //     };
//     //     let loop_instrs =  vec![W::Instruction::If(loop_inner_if)];
//     //
//     //     W::Loop {
//     //         label: None,
//     //         block_type: WT::UNITYPE,
//     //         instructions: loop_instrs,
//     //     }
//     // }
//     //
//     // /// Turns a Ruby Expr into a Wasm predicate.
//     // /// A ruby Expr evaluates to a ruby-value (True, False, Nil, ...)
//     // /// To use as a Wasm predicate, we need to test whether the result is truthy or not.
//     // /// TODO -- right now we pretend that "truthy" is "not-false"
//     // fn predicate(&mut self, expr: R::Expr) -> Vec<W::Instruction> {
//     //     let mut instrs = self.ruby_expr(expr);
//     //     instrs.append(&mut runtime::is_false());
//     //     instrs.push(W::Instruction::I32Eqz);
//     //     instrs
//     // }
// }
//
//
// // fn const_i31(n: WV::I32) -> Vec<W::Instruction> {
// //     vec![W::Instruction::ConstI32(n), W::Instruction::RefI31]
// // }

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
    //      (then statements)))
    let R::While {
        predicate,
        statements,
    } = while_expr;

    builder.loop_(|builder| {
        builder.if_else(
            ctx,
            |ctx, builder| {
                compile_expr_to_wasm_predicate(ctx, builder, predicate);
            },
            |ctx, builder| compile_statements(ctx, builder, statements),
            |ctx, builder| {
                // Just return nil
                const_i31(builder, 0b0101)
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

    builder.loop_(|builder| {
        builder.if_else(
            ctx,
            |ctx, builder| {
                compile_expr_to_wasm_predicate(ctx, builder, predicate);
                // `binary_not ≡ eqz` when result is interpreted as boolean
                builder.unop(UnaryOp::I32Eqz);
            },
            |ctx, builder| compile_statements(ctx, builder, statements),
            |ctx, builder| {
                // Just return nil
                const_i31(builder, 0b0101)
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
    // instrs.append(&mut runtime::is_false());
    // instrs.push(W::Instruction::I32Eqz);
    // instrs
}

fn const_i31<A: ArenaProvider>(builder: &mut InstrSeqBuilder<A>, val: i32) {
    builder.i32_const(val).unop(UnaryOp::RefI31);
}
