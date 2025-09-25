// use crate::wasm::types::{
//     GlobalType, HeapType, Mutability, NumberType, ReferenceType, Type, UNITYPE, ValType,
// };
// use crate::wasm::{Expr, Function, FunctionIdx, Global, GlobalIdx, If, Instruction, Loop, Module};
// use std::fmt::Write;
// use std::iter;
// use std::ptr::write;
//
// pub struct WatPrinter {
//     output: String,
//     indent: usize,
// }
//
// // TODO -- Each print_ function should assume that it's in the correct start location when called.
// impl WatPrinter {
//     pub fn new() -> Self {
//         WatPrinter {
//             output: String::new(),
//             indent: 0,
//         }
//     }
//
//     pub fn print_module(mut self, module: &Module) -> String {
//         self.indent();
//         write!(self.output, "(module").unwrap();
//         self.indent += 2;
//         self.print_exports(module);
//         self.print_globals(&module.globals);
//         self.print_functions(module);
//         self.print_start_function(module);
//         writeln!(self.output, ")").unwrap();
//         self.indent -= 2;
//         debug_assert_eq!(self.indent, 0);
//
//         self.output
//     }
//
//     fn print_exports(&mut self, module: &Module) {
//         for export in &module.exports {
//             self.print_export(export);
//         }
//     }
//
//     fn print_export(&mut self, export: &FunctionIdx) {
//         writeln!(self.output).unwrap();
//         self.indent();
//         match export {
//             FunctionIdx::Index(_) => panic!(),
//             FunctionIdx::Id(name) => {
//                 write!(self.output, "(export \"_start\" (func ${}))", name).unwrap()
//             }
//         }
//     }
//
//     fn print_globals(&mut self, globals: &Vec<Global>) {
//         for global in globals {
//             self.print_global(global);
//         }
//     }
//
//     /// Pre: "cursor" is on the line before this global
//     ///     Indent is correct for this global
//     fn print_global(&mut self, global: &Global) {
//         match &global.id {
//             None => {
//                 todo!("Indexed refs")
//             }
//             Some(id) => {
//                 writeln!(self.output).unwrap();
//                 self.indent();
//                 write!(self.output, "(global {} ", id).unwrap();
//                 self.print_global_type(&global.global_type);
//                 self.indent += 2;
//                 self.print_expr(&global.expr);
//                 write!(self.output, ")").unwrap();
//                 self.indent -= 2;
//             }
//         }
//     }
//
//     fn print_functions(&mut self, module: &Module) {
//         for function in &module.functions {
//             self.print_function(function)
//         }
//     }
//
//     fn print_function(&mut self, function: &Function) {
//         writeln!(self.output).unwrap();
//         self.indent();
//         write!(self.output, "(func").unwrap();
//         if let Some(name) = &function.id {
//             write!(self.output, " ${} ", name).unwrap();
//         }
//         write!(self.output, " (result ").unwrap();
//         self.print_type(&UNITYPE);
//         writeln!(self.output, ")").unwrap();
//         self.indent += 2;
//         self.indent();
//         self.print_expr(&function.body);
//         write!(self.output, ")").unwrap();
//         self.indent -= 2;
//     }
//
//     fn print_expr(&mut self, body: &Expr) {
//         self.print_instructions(&body.0)
//     }
//
//     fn print_instructions(&mut self, instructions: &[Instruction]) {
//         let mut instr_iter = instructions.iter();
//         let instr = instr_iter.next().unwrap();
//         self.print_instruction(instr);
//
//         self.indent += 2;
//         for instr in instr_iter {
//             writeln!(self.output).unwrap();
//             self.indent();
//             self.print_instruction(instr);
//         }
//         self.indent -= 2;
//     }
//
//     fn print_instruction(&mut self, instr: &Instruction) {
//         match instr {
//             Instruction::ConstI32(n) => write!(self.output, "i32.const {}", n).unwrap(),
//             Instruction::ConstI64(n) => write!(self.output, "i64.const {}", n).unwrap(),
//             Instruction::RefI31 => write!(self.output, "ref.i31").unwrap(),
//             Instruction::I31GetU => write!(self.output, "i31.get_u").unwrap(),
//             Instruction::I32Or => write!(self.output, "i32.or").unwrap(),
//             Instruction::I32Eqz => write!(self.output, "i32.eqz").unwrap(),
//             Instruction::I32Eq => write!(self.output, "i32.eq").unwrap(),
//             Instruction::GlobalGet(idx) => match idx {
//                 &GlobalIdx::Idx(_idx) => {
//                     todo!("Indexed refs")
//                 }
//                 GlobalIdx::Id(id) => write!(self.output, "(global.get {})", id).unwrap(),
//             },
//             Instruction::If(if_instr) => self.print_if(if_instr),
//             Instruction::I32Xor => write!(self.output, "i32.xor").unwrap(),
//             Instruction::Loop(loop_instr) => self.print_loop(loop_instr),
//         }
//     }
//
//     fn print_if(&mut self, if_instr: &If) {
//         let If {
//             label,
//             block_type,
//             predicate_instrs,
//             then_instrs,
//             else_instrs,
//         } = if_instr;
//
//         let label_str = match label {
//             None => "".to_string(),
//             Some(label) => format!("{} ", label),
//         };
//         write!(self.output, "(if {}(result ", label_str).unwrap();
//         self.print_type(block_type);
//         write!(self.output, ")").unwrap();
//         self.indent += 2;
//         writeln!(self.output).unwrap();
//         self.indent();
//         write!(self.output, "(").unwrap();
//         self.print_instructions(predicate_instrs);
//         writeln!(self.output, ")").unwrap();
//         self.indent();
//         write!(self.output, "(then ").unwrap();
//         self.print_instructions(then_instrs);
//         write!(self.output, ")").unwrap();
//
//         self.indent();
//         write!(self.output, "\n(else ").unwrap();
//         self.print_instructions(else_instrs);
//         write!(self.output, ")").unwrap();
//
//         self.indent -= 2;
//         write!(self.output, ")").unwrap();
//     }
//
//     fn print_loop(&mut self, loop_instr: &Loop) {
//         let Loop {
//             label,
//             block_type,
//             instructions,
//         } = loop_instr;
//
//         let label_str = match label {
//             None => "".to_string(),
//             Some(label) => format!("{} ", label),
//         };
//
//         write!(self.output, "(loop {}(result ", label_str).unwrap();
//         self.print_type(block_type);
//         write!(self.output, ")").unwrap();
//         self.indent += 2;
//         writeln!(self.output).unwrap();
//         self.indent();
//         self.print_instructions(instructions);
//         write!(self.output, ")").unwrap();
//         self.indent -= 2;
//     }
//
//     fn print_start_function(&mut self, module: &Module) {
//         if let Some(start_idx) = &module.start {
//             writeln!(self.output).unwrap();
//             self.indent();
//             write!(self.output, "(start").unwrap();
//             match start_idx {
//                 &FunctionIdx::Index(_idx) => {
//                     todo!("Indexed refs")
//                 }
//                 FunctionIdx::Id(name) => {
//                     write!(self.output, " ${}", name).unwrap();
//                 }
//             }
//             write!(self.output, ")").unwrap()
//         }
//     }
//
//     fn print_type(&mut self, wasm_type: &Type) {
//         match wasm_type {
//             Type::Val(value_type) => self.print_value_type(value_type),
//             Type::ReferenceType(ref_type) => self.print_reference_type(ref_type),
//         }
//     }
//
//     /// See: Wasm reference 6.4.14
//     fn print_global_type(&mut self, global_type: &GlobalType) {
//         match global_type.mutability {
//             Mutability::Const => self.print_value_type(&global_type.value_type),
//             Mutability::Var => {
//                 write!(self.output, "(mut ").unwrap();
//                 self.print_value_type(&global_type.value_type);
//                 write!(self.output, " )").unwrap()
//             }
//         }
//     }
//
//     fn print_value_type(&mut self, value_type: &ValType) {
//         match value_type {
//             ValType::NumberType(number_type) => self.print_number_type(number_type),
//         }
//     }
//
//     fn print_number_type(&mut self, number_type: &NumberType) {
//         let str = match number_type {
//             NumberType::I32 => "i32",
//             NumberType::I64 => "i64",
//             NumberType::F32 => "f32",
//             NumberType::F64 => "f64",
//         };
//         write!(self.output, "{}", str).unwrap()
//     }
//
//     fn print_reference_type(&mut self, reference_type: &ReferenceType) {
//         let ReferenceType { null, heap_type } = reference_type;
//
//         let null = if *null { "null " } else { "" };
//         write!(self.output, "(ref {}", null).unwrap();
//         self.print_heap_type(heap_type);
//         write!(self.output, ")").unwrap();
//     }
//
//     fn print_heap_type(&mut self, heap_type: &HeapType) {
//         let str = match heap_type {
//             HeapType::Eq => "eq",
//         };
//         write!(self.output, "{}", str).unwrap();
//     }
//
//     fn indent(&mut self) {
//         write!(
//             self.output,
//             "{}",
//             iter::repeat(' ').take(self.indent).collect::<String>()
//         )
//         .unwrap()
//     }
// }

use crate::wasm::function::{Function, InstrSeq, InstrSeqId};
use crate::wasm::module::{Module, ModuleFunctions};
use crate::wasm::types::{AbsHeapType, BlockType, HeapType, NumberType, ParamType, RefType, ResultType, ValType, UNITYPE};
use crate::wasm::{Block, Const, IfElse, Instr, Loop, UnaryOp, Unop, Value};
use crate::FunctionBuilder;
use id_arena::Arena;
use pretty::{Doc, RcDoc};
use std::hash::Hash;

impl Module {
    pub fn to_pretty(&self) -> String {
        let mut w = Vec::new();
        module_to_doc(self).render(2, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

fn module_to_doc(module: &Module) -> RcDoc<'static> {
    let Module {
        funcs,
        start,
        name: _name,
    } = module;

    let start = match start {
        None => None,
        &Some(start) => Some(funcs.get(start)),
    };

    let child_docs: Box<[RcDoc<'static>]> = [
        Some(module_functions_to_doc(funcs)),
        start.map(|f| function_to_doc(f)),
    ]
    .into_iter()
    .filter_map(|doc| doc)
    .collect();

    RcDoc::text("(")
        .append(RcDoc::text("module"))
        .append(RcDoc::intersperse(child_docs, Doc::line()).nest(1))
        .append(RcDoc::text(")"))
}

fn module_functions_to_doc(module_functions: &ModuleFunctions) -> RcDoc<'static> {
    let funcs: Box<[RcDoc<'static>]> = module_functions.iter().map(|f| function_to_doc(f)).collect();
    RcDoc::intersperse(funcs, Doc::line())
}

fn function_to_doc(f: &Function) -> RcDoc<'static> {
    let FunctionBuilder {
        instr_seq_arena,
        entry_point,
        ..
    } = &f.builder;

    let params: Vec<RcDoc<'static>> = f.params().iter().map(|p| param_to_doc(p)).collect();
    let results: Vec<RcDoc<'static>> = f.results().iter().map(|r| result_type_to_doc(r)).collect();
    let params_results: Box<[RcDoc<'static>]> =
        params.into_iter().chain(results.into_iter()).collect();

    let instr_seq_doc = instr_seq_to_doc(instr_seq_arena, *entry_point);

    RcDoc::text("(").append(
        RcDoc::intersperse(params_results, Doc::line())
            .nest(1)
            .group(),
    ).append(instr_seq_doc).nest(1)
        .append(")")
}

fn instr_seq_to_doc(arena: &Arena<InstrSeq>, seq_id: InstrSeqId) -> RcDoc<'static> {
    let seq = arena.get(seq_id).unwrap();
    let instrs: Box<[RcDoc<'static>]> = seq.0.iter().map(|instr| instr_to_doc(arena, instr)).collect();

    RcDoc::intersperse(instrs, RcDoc::line())
}

fn instr_to_doc(instr_seq_arena: &Arena<InstrSeq>, instr: &Instr) -> RcDoc<'static> {
    match instr {
        Instr::Block(b) => block_to_doc(instr_seq_arena, b),
        Instr::Loop(l) => loop_to_doc(instr_seq_arena, l),
        Instr::Const(c) => const_to_doc(c),
        Instr::Unop(u) => unop_to_doc(u),
        Instr::IfElse(i) => if_else_to_doc(instr_seq_arena, i),
    }
}

fn block_to_doc(instr_seq_arena: &Arena<InstrSeq>, block: &Block) -> RcDoc<'static> {
    let &Block { seq } = block;
    RcDoc::text("(block")
        .append(block_type_to_doc(&UNITYPE.into_block_type_result()))
        .append(RcDoc::line())
        .append(instr_seq_to_doc(instr_seq_arena, seq)).nest(1)
        .append(")")
}

fn loop_to_doc(instr_seq_arena: &Arena<InstrSeq>, l: &Loop) -> RcDoc<'static> {
    let &Loop { seq } = l;
    RcDoc::text("(loop")
        .append(block_type_to_doc(&UNITYPE.into_block_type_result()))
        .append(RcDoc::line())
        .append(instr_seq_to_doc(instr_seq_arena, seq)).nest(1)
        .append(")")
}

fn const_to_doc(c: &Const) -> RcDoc<'static> {
    let Const { value } = c;
    let ty = match value {
        Value::I32(_) => RcDoc::text("i32.const".to_owned()),
        Value::I64(_) => RcDoc::text("i64.const".to_owned()),
        Value::F32(_) =>RcDoc::text("f32.const".to_owned()),
        Value::F64(_) =>RcDoc::text("f64.const".to_owned()),
    };

    let value = match value {
        Value::I32(n) => RcDoc::text(format!("{}", n)),
        Value::I64(n) => RcDoc::text(format!("{}", n)),
        Value::F32(n) =>RcDoc::text(format!("{}", n)),
        Value::F64(n) =>RcDoc::text(format!("{}", n)),
    };

    RcDoc::text("(")
        .append(ty)
        .append(value)
        .append(RcDoc::text(")"))
}

fn unop_to_doc(unop: &Unop) -> RcDoc<'static> {
    let Unop { op } = unop;
    match op {
        UnaryOp::RefI31 => RcDoc::text("(ref.i31)")
    }
}

fn if_else_to_doc(instr_seq_arena: &Arena<InstrSeq>, if_else: &IfElse) -> RcDoc<'static> {
    let &IfElse { consequent, alternative } = if_else;

    let consequent = instr_seq_to_doc(instr_seq_arena, consequent);
    let alternative = instr_seq_to_doc(instr_seq_arena, alternative);

    RcDoc::text("(if")
        .append(block_type_to_doc(&UNITYPE.into_block_type_result()))
        .append(RcDoc::line())
        .append(consequent).nest(1)
        .append(RcDoc::line())
        .append(alternative).nest(1)
        .append(")")
}

fn param_to_doc(param: &ParamType) -> RcDoc<'static> {
    let ParamType { name, ty } = param;

    let child_docs: Box<[RcDoc<'static>]> = Box::new([RcDoc::text(name.to_owned()), val_type_to_doc(ty)]);

    RcDoc::text("(param")
        .append(RcDoc::intersperse(child_docs, Doc::line()).nest(1).group())
        .append(")")
}

fn block_type_to_doc(ty: &BlockType) -> RcDoc<'static> {
    match ty {
        BlockType::Result(ty) => result_type_to_doc(ty),
        BlockType::TypeUse(ident) => RcDoc::text(ident.to_owned())
    }
}

fn result_type_to_doc(ty: &ResultType) -> RcDoc<'static> {
    let ResultType(ty) = ty;

    RcDoc::text("(param")
        .append(val_type_to_doc(ty))
        .append(")")
}

fn val_type_to_doc(ty: &ValType) -> RcDoc<'static> {
    match ty {
        ValType::NumberType(ty) => num_type_to_doc(ty),
        ValType::Ref(ty) => ref_type_to_doc(ty),
    }
}

fn num_type_to_doc(ty: &NumberType) -> RcDoc<'static> {
    RcDoc::text(match ty {
        NumberType::I32 => "i32",
        NumberType::I64 => "i64",
        NumberType::F32 => "f32",
        NumberType::F64 => "f64"
    })
}

fn ref_type_to_doc(ty: &RefType) -> RcDoc<'static> {
    let RefType { nullable, heap_type } = ty;
    if *nullable {
        RcDoc::text("(ref")
            .append(heap_type_to_doc(heap_type))
            .append(")")
    } else {
        RcDoc::text("(ref null")
            .append(heap_type_to_doc(heap_type))
            .append(")")
    }
}

fn heap_type_to_doc(ty: &HeapType) -> RcDoc<'static> {
    match ty {
        HeapType::Abstract(ty) => abs_heap_type_to_doc(ty),
        HeapType::Identifier(ident) => RcDoc::text(ident.to_owned())
    }
}

fn abs_heap_type_to_doc(ty: &AbsHeapType) -> RcDoc<'static> {
    RcDoc::text(match ty {
        AbsHeapType::Func => "func",
        AbsHeapType::Extern => "extern",
        AbsHeapType::Any => "any",
        AbsHeapType::None => "none",
        AbsHeapType::NoExtern => "noextern",
        AbsHeapType::NoFunc => "nofunc",
        AbsHeapType::Eq => "eq",
        AbsHeapType::Struct => "struct",
        AbsHeapType::Array => "array",
        AbsHeapType::I31 => "i31",
        AbsHeapType::Exn => "exn",
        AbsHeapType::NoExn => "noexn",
    })
}
