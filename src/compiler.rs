//! Compiles a Ruby AST to a Wasm module
//!
//! NB: Rust signed <-> unsigned casts (using `as`) reinterpret with 2's complement.
//! ```
//! assert_eq!(-1, u64::MAX as i64);
//! ```

// R for Ruby
use crate::node as R;
// W for Wasm
use crate::wasm as W;
use W::types as WT;
use W::values as WV;
use crate::wasm::GlobalIdx;

const RUBY_TOP_LEVEL_FUNCTION_NAME: &str = "__ruby_top_level_function";

/// We give fixnums half an i31, marking MSB 1
/// (0b1xx_xxxx...): i31
const FIXNUM_BIT_WIDTH: u32 = 30;

/// Fixnums are identified with a 1 in the MSB of the i31
pub const FIXNUM_MARKER: i32 = 1 << 30;

pub struct Compiler {
    globals: Vec<W::Global>
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            globals: vec![]
        }
    }

    pub fn compile(self, program: R::Program) -> W::Module {
        // Current strategy:
        // We're kinda doing recursive descent into the Wasm module structure.
        // - There is one wasm_module per ruby_program
        // - There is one wasm_function per ruby_program
        //  - `program.statements.body.map(|expr| => expr.to_wasm_instrs())`

        self.module(program)
    }

    fn module(mut self, program: R::Program) -> W::Module {
        let functions = vec![self.function(program.statements)];

        // Export our top-level function
        let exports = vec![W::FunctionIdx::Id(RUBY_TOP_LEVEL_FUNCTION_NAME.to_string())];

        W::Module {
            functions,
            exports,
            globals: self.globals,
            start: None,
        }
    }

    fn function(&mut self, statements: R::Statements) -> W::Function {
        let body = self.expr(statements);
        W::Function {
            id: Some(RUBY_TOP_LEVEL_FUNCTION_NAME.to_string()),
            body,
        }
    }

    fn expr(&mut self, statements: R::Statements) -> W::Expr {
        let mut expr_instrs: Vec<W::Instruction> = vec![];

        for ruby_expr in statements.body {
            let mut instrs = self.compile_ruby_expr(ruby_expr);
            expr_instrs.append(&mut instrs);
        }

        W::Expr(expr_instrs)
    }

    fn compile_ruby_expr(&mut self, ruby_expr: R::Expr) -> Vec<W::Instruction> {

        match ruby_expr {
            R::Expr::Integer(n) => self.integer(n),
            R::Expr::True => const_i31(WV::I32::TRUE),
            R::Expr::False => const_i31(WV::I32::FALSE),
            R::Expr::Nil => const_i31(WV::I32::NIL),
        }
    }

    /// Convert the given integer into a Wasm fixnum or const global representation
    fn integer(&mut self, n: i64) -> Vec<W::Instruction> {
        // Strategy:
        // Determine whether we're in range of a fixnum
        // If not, add a const global int and get it

        /// Minimum size required for 2's complement representation of the given number
        /// Strategy from:
        /// https://internals.rust-lang.org/t/add-methods-that-return-the-number-of-bits-necessary-to-represent-an-integer-in-binary-to-the-standard-library/21870/7
        fn bit_width(n: i64) -> u32 {
            i64::BITS - n.abs().leading_zeros() + 1
        }

        // If if-let guards were stable I'd use those.
        match n {
            n if bit_width(n) <= FIXNUM_BIT_WIDTH => {
                let fixnum = FIXNUM_MARKER | i32::try_from(n).unwrap();
                let fixnum = WV::I32(fixnum as u32);
                vec![W::Instruction::ConstI32(fixnum), W::Instruction::RefI31]
            }
            n if bit_width(n) <= i32::BITS => {
                let n = i32::try_from(n).unwrap();
                // TODO -- Need to intern this.
                //  If you have 2 of the same-valued global it probably breaks at validation time.
                //  Also, does this introduce illegal identifier characters?
                let global_id = format!("$global-i32-{}", n);
                let expr_instrs = vec![W::Instruction::ConstI32(WV::I32(n as u32))];

                let global = W::Global {
                    id: Some(global_id.clone()),
                    global_type: WT::GlobalType {
                        mutability: WT::Mutability::Const,
                        value_type: WT::ValueType::NumberType(WT::NumberType::I32),
                    },
                    expr: W::Expr(expr_instrs)
                };
                self.globals.push(global);
                vec![W::Instruction::GlobalGet(GlobalIdx::Id(global_id))]
            }
            // Guard is here for when bignums are implemented.
            n if bit_width(n) <= i64::BITS => {
                let n = i64::try_from(n).unwrap();
                // TODO -- Need to intern this.
                //  If you have 2 of the same-valued global it probably breaks at validation time.
                //  Also, does this introduce illegal identifier characters?
                let global_id = format!("$global-i64-{}", n);
                let expr_instrs = vec![W::Instruction::ConstI64(WV::I64(n as u64))];

                let global = W::Global {
                    id: Some(global_id.clone()),
                    global_type: WT::GlobalType {
                        mutability: WT::Mutability::Const,
                        value_type: WT::ValueType::NumberType(WT::NumberType::I64),
                    },
                    expr: W::Expr(expr_instrs)
                };
                self.globals.push(global);
                vec![W::Instruction::GlobalGet(GlobalIdx::Id(global_id))]
            }
            _ => {
                todo!("Bignums not yet implemented.
                      [n={:x}] larger than W::I64",
                      22)
            }
        }
    }
}


fn const_i31(n: WV::I32) -> Vec<W::Instruction> {
    vec![W::Instruction::ConstI32(n), W::Instruction::RefI31]
}
