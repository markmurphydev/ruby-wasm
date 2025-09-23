use crate::wasm::types::{
    GlobalType, HeapType, Mutability, NumberType, ReferenceType, Type, UNITYPE, ValType,
};
use crate::wasm::{Expr, Function, FunctionIdx, Global, GlobalIdx, If, Instruction, Loop, Module};
use std::fmt::Write;
use std::iter;
use std::ptr::write;

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
        debug_assert_eq!(self.indent, 0);

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
            write!(self.output, " ${} ", name).unwrap();
        }
        write!(self.output, " (result ").unwrap();
        self.print_type(&UNITYPE);
        writeln!(self.output, ")").unwrap();
        self.indent += 2;
        self.indent();
        self.print_expr(&function.body);
        write!(self.output, ")").unwrap();
        self.indent -= 2;
    }

    fn print_expr(&mut self, body: &Expr) {
        self.print_instructions(&body.0)
    }

    fn print_instructions(&mut self, instructions: &[Instruction]) {
        let mut instr_iter = instructions.iter();
        let instr = instr_iter.next().unwrap();
        self.print_instruction(instr);

        self.indent += 2;
        for instr in instr_iter {
            writeln!(self.output).unwrap();
            self.indent();
            self.print_instruction(instr);
        }
        self.indent -= 2;
    }

    fn print_instruction(&mut self, instr: &Instruction) {
        match instr {
            Instruction::ConstI32(n) => write!(self.output, "i32.const {}", n).unwrap(),
            Instruction::ConstI64(n) => write!(self.output, "i64.const {}", n).unwrap(),
            Instruction::RefI31 => write!(self.output, "ref.i31").unwrap(),
            Instruction::I31GetU => write!(self.output, "i31.get_u").unwrap(),
            Instruction::I32Or => write!(self.output, "i32.or").unwrap(),
            Instruction::I32Eqz => write!(self.output, "i32.eqz").unwrap(),
            Instruction::I32Eq => write!(self.output, "i32.eq").unwrap(),
            Instruction::GlobalGet(idx) => match idx {
                &GlobalIdx::Idx(_idx) => {
                    todo!("Indexed refs")
                }
                GlobalIdx::Id(id) => write!(self.output, "(global.get {})", id).unwrap(),
            },
            Instruction::If(if_instr) => self.print_if(if_instr),
            Instruction::I32Xor => write!(self.output, "i32.xor").unwrap(),
            Instruction::Loop(loop_instr) => self.print_loop(loop_instr),
        }
    }

    fn print_if(&mut self, if_instr: &If) {
        let If {
            label,
            block_type,
            predicate_instrs,
            then_instrs,
            else_instrs,
        } = if_instr;

        let label_str = match label {
            None => "".to_string(),
            Some(label) => format!("{} ", label),
        };
        write!(self.output, "(if {}(result ", label_str).unwrap();
        self.print_type(block_type);
        write!(self.output, ")").unwrap();
        self.indent += 2;
        writeln!(self.output).unwrap();
        self.indent();
        write!(self.output, "(").unwrap();
        self.print_instructions(predicate_instrs);
        writeln!(self.output, ")").unwrap();
        self.indent();
        write!(self.output, "(then ").unwrap();
        self.print_instructions(then_instrs);
        write!(self.output, ")").unwrap();

        self.indent();
        write!(self.output, "\n(else ").unwrap();
        self.print_instructions(else_instrs);
        write!(self.output, ")").unwrap();

        self.indent -= 2;
        write!(self.output, ")").unwrap();
    }

    fn print_loop(&mut self, loop_instr: &Loop) {
        let Loop {
            label,
            block_type,
            instructions,
        } = loop_instr;

        let label_str = match label {
            None => "".to_string(),
            Some(label) => format!("{} ", label),
        };

        write!(self.output, "(loop {}(result ", label_str).unwrap();
        self.print_type(block_type);
        write!(self.output, ")").unwrap();
        self.indent += 2;
        writeln!(self.output).unwrap();
        self.indent();
        self.print_instructions(instructions);
        write!(self.output, ")").unwrap();
        self.indent -= 2;
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

    fn print_type(&mut self, wasm_type: &Type) {
        match wasm_type {
            Type::Val(value_type) => self.print_value_type(value_type),
            Type::ReferenceType(ref_type) => self.print_reference_type(ref_type),
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

    fn print_value_type(&mut self, value_type: &ValType) {
        match value_type {
            ValType::NumberType(number_type) => self.print_number_type(number_type),
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

    fn print_reference_type(&mut self, reference_type: &ReferenceType) {
        let ReferenceType { null, heap_type } = reference_type;

        let null = if *null { "null " } else { "" };
        write!(self.output, "(ref {}", null).unwrap();
        self.print_heap_type(heap_type);
        write!(self.output, ")").unwrap();
    }

    fn print_heap_type(&mut self, heap_type: &HeapType) {
        let str = match heap_type {
            HeapType::Eq => "eq",
        };
        write!(self.output, "{}", str).unwrap();
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
