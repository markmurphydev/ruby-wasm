use crate::wasm::{Expr, Function, FunctionIndex, Instruction, Module};
use std::fmt::Write;
use std::iter;

pub struct Printer {
    output: String,
    indent: usize,
}

impl Printer {
    pub fn new() -> Self {
        Printer {
            output: String::new(),
            indent: 0,
        }
    }

    pub fn print_module(mut self, module: &Module) -> String {
        self.indent();
        write!(self.output, "(module").unwrap();
        self.indent += 2;
        self.print_exports(module);
        self.print_functions(module);
        self.print_start_function(module);
        writeln!(self.output, ")").unwrap();
        self.indent -= 2;
        assert_eq!(self.indent, 0);

        self.output
    }

    fn print_exports(&mut self, module: &Module) {
        for export in &module.exports {
            self.print_export(export);
        }
    }

    fn print_export(&mut self, export: &FunctionIndex) {
        writeln!(self.output).unwrap();
        self.indent();
        match export {
            FunctionIndex::Index(_) => panic!(),
            FunctionIndex::Name(name) => {
                write!(self.output, "(export \"_start\" (func ${}))", name).unwrap()
            }
        }
    }

    fn print_functions(&mut self, module: &Module) {
        for function in &module.functions {
            self.print_function(function)
        }
    }

    fn print_function(&mut self, function: &Function) {
        writeln!(self.output).unwrap();
        self.indent();
        write!(self.output, "(func").unwrap();
        if let Some(name) = &function.name {
            write!(self.output, " ${}", name).unwrap();
        }
        write!(self.output, " (result (ref i31))").unwrap();
        self.indent += 2;
        self.print_function_body(&function.body);
        write!(self.output, ")").unwrap();
        self.indent -= 2;
    }

    fn print_function_body(&mut self, body: &Expr) {
        for instr in body.0.iter() {
            self.print_instr(instr);
        }
    }

    fn print_instr(&mut self, instr: &Instruction) {
        writeln!(self.output).unwrap();
        self.indent();
        match instr {
            Instruction::ConstI32(n) => write!(self.output, "i32.const {}", n).unwrap(),
            Instruction::RefI31 => write!(self.output, "ref.i31").unwrap(),
        }
    }

    fn print_start_function(&mut self, module: &Module) {
        if let Some(start_idx) = &module.start {
            writeln!(self.output).unwrap();
            self.indent();
            write!(self.output, "(start").unwrap();
            match start_idx {
                FunctionIndex::Index(idx) => {
                    write!(self.output, " {}", idx).unwrap();
                }
                FunctionIndex::Name(name) => {
                    write!(self.output, " ${}", name).unwrap();
                }
            }
            write!(self.output, ")").unwrap();
        }
    }

    fn indent(&mut self) {
        write!(
            self.output,
            "{}",
            iter::repeat(' ').take(self.indent).collect::<String>()
        )
        .unwrap();
    }
}
