//! Compiles a Ruby AST to a Wasm module
//! Extremely ad-hoc. I have no theory yet of how the mapping should happen.
//! But it's a walking skeleton!

// R for Ruby
use crate::node as R;
// W for Wasm
use crate::wasm as W;

const RUBY_TOP_LEVEL_FUNCTION_NAME: &str = "__ruby_top_level_function";

pub struct Compiler;

impl Compiler {
    pub fn compile(&mut self, program: R::Program) -> W::Module {
        // Current strategy:
        // We're kinda doing recursive descent into the Wasm module structure.
        // - There is one wasm_module per ruby_program
        // - There is one wasm_function per ruby_program
        //  - `program.statements.body.map(|expr| => expr.to_wasm_instrs())`

        self.module(program)
    }

    fn module(&mut self, program: R::Program) -> W::Module {
        let functions = vec![self.function(program.statements)];

        // Export our top-level function
        let exports = vec![W::FunctionIndex::Name(RUBY_TOP_LEVEL_FUNCTION_NAME.to_string())];

        W::Module {
            functions,
            exports,
            start: None,
        }
    }

    fn function(&mut self, statements: R::Statements) -> W::Function {
        let body = self.expr(statements);
        W::Function {
            name: Some(RUBY_TOP_LEVEL_FUNCTION_NAME.to_string()),
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

        fn const_i31(n: W::Integer) -> Vec<W::Instruction> {
            vec![W::Instruction::ConstI32(n), W::Instruction::RefI31]
        }

        match ruby_expr {
            R::Expr::True => const_i31(W::Integer::TRUE),
            R::Expr::False => const_i31(W::Integer::FALSE),
            R::Expr::Nil => const_i31(W::Integer::NIL),
        }
    }
}