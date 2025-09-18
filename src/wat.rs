use crate::wasm::types::{GlobalType, Mutability, NumberType, Type, ValueType};
use crate::wasm::{Expr, Function, FunctionIdx, Global, GlobalIdx, Instruction, Module};
use std::fmt::Write;
use std::iter;

pub struct WatPrinter {
    output: String,
    indent: usize,
}

// TODO -- Each print_ function should assume that it's in the correct start location when called.
impl WatPrinter {
    pub fn new() -> Self {
        WatPrinter {
            output: String::new(),
            indent: 0,
        }
    }

    pub fn print_module(mut self, module: &Module) -> String {
        self.indent();
        write!(self.output, "(module").unwrap();
        self.indent += 2;
        self.print_exports(module);
        self.print_globals(&module.globals);
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

    fn print_export(&mut self, export: &FunctionIdx) {
        writeln!(self.output).unwrap();
        self.indent();
        match export {
            FunctionIdx::Index(_) => panic!(),
            FunctionIdx::Id(name) => {
                write!(self.output, "(export \"_start\" (func ${}))", name).unwrap()
            }
        }
    }

    fn print_globals(&mut self, globals: &Vec<Global>) {
        for global in globals {
            self.print_global(global);
        }
    }

    /// Pre: "cursor" is on the line before this global
    ///     Indent is correct for this global
    fn print_global(&mut self, global: &Global) {
        match &global.id {
            None => {
                todo!("Indexed refs")
            }
            Some(id) => {
                writeln!(self.output).unwrap();
                self.indent();
                write!(self.output, "(global {} ", id).unwrap();
                self.print_global_type(&global.global_type);
                self.indent += 2;
                self.print_expr(&global.expr);
                write!(self.output, ")").unwrap();
                self.indent -= 2;
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
        if let Some(name) = &function.id {
            write!(self.output, " ${}", name).unwrap();
        }
        write!(self.output, " (result i32)").unwrap();
        self.indent += 2;
        self.print_expr(&function.body);
        write!(self.output, ")").unwrap();
        self.indent -= 2;
    }

    fn print_expr(&mut self, body: &Expr) {
        for instr in body.0.iter() {
            self.print_instr(instr);
        }
    }

    fn print_instr(&mut self, instr: &Instruction) {
        writeln!(self.output).unwrap();
        self.indent();
        match instr {
            Instruction::ConstI32(n) => write!(self.output, "i32.const {}", n).unwrap(),
            Instruction::ConstI64(n) => write!(self.output, "i64.const {}", n).unwrap(),
            Instruction::RefI31 => write!(self.output, "ref.i31").unwrap(),
            Instruction::GlobalGet(idx) => match idx {
                &GlobalIdx::Idx(_idx) => {
                    todo!("Indexed refs")
                }
                GlobalIdx::Id(id) => {
                    write!(self.output, "(global.get {})", id).unwrap()
                }
            },
            &Instruction::If(_) => todo!(),
        }
    }

    fn print_start_function(&mut self, module: &Module) {
        if let Some(start_idx) = &module.start {
            writeln!(self.output).unwrap();
            self.indent();
            write!(self.output, "(start").unwrap();
            match start_idx {
                &FunctionIdx::Index(_idx) => {
                    todo!("Indexed refs")
                }
                FunctionIdx::Id(name) => {
                    write!(self.output, " ${}", name).unwrap();
                }
            }
            write!(self.output, ")").unwrap()
        }
    }

    /// See: Wasm reference 6.4.14
    fn print_global_type(&mut self, global_type: &GlobalType) {
        match global_type.mutability {
            Mutability::Const => self.print_value_type(&global_type.value_type),
            Mutability::Var => {
                write!(self.output, "(mut ").unwrap();
                self.print_value_type(&global_type.value_type);
                write!(self.output, " )").unwrap()
            }
        }
    }

    fn print_value_type(&mut self, value_type: &ValueType) {
        match value_type {
            ValueType::NumberType(number_type) => self.print_number_type(number_type),
        }
    }

    fn print_number_type(&mut self, number_type: &NumberType) {
        let str = match number_type {
            NumberType::I32 => "i32",
            NumberType::I64 => "i64",
            NumberType::F32 => "f32",
            NumberType::F64 => "f64",
        };
        write!(self.output, "{}", str).unwrap()
    }

    fn indent(&mut self) {
        write!(
            self.output,
            "{}",
            iter::repeat(' ').take(self.indent).collect::<String>()
        )
        .unwrap()
    }
}
