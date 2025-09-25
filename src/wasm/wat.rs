use crate::FunctionBuilder;
use crate::wasm::function::{Function, InstrSeq, InstrSeqId};
use crate::wasm::module::{Module, ModuleFunctions};
use crate::wasm::types::{
    AbsHeapType, BlockType, HeapType, NumberType, ParamType, RefType, ResultType, UNITYPE, ValType,
};
use crate::wasm::{BinaryOp, Binop, Block, Const, IfElse, Instr, Loop, UnaryOp, Unop, Value};
use id_arena::Arena;
use pretty::{Doc, RcDoc};

impl Module {
    pub fn to_pretty(&self) -> String {
        let mut w = Vec::new();
        module_to_doc(self).render(80, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

fn module_to_doc(module: &Module) -> RcDoc<'static> {
    let Module {
        globals,
        funcs,
        start,
    } = module;

    let start = match start {
        None => None,
        &Some(start) => Some(funcs.get(start)),
    };

    let child_docs = [
        Some(module_functions_to_doc(funcs)),
        start.map(|f| function_to_doc(f)),
    ]
    .into_iter()
    .filter_map(|doc| doc);

    RcDoc::text("(module")
        .append(RcDoc::line())
        .append(RcDoc::intersperse(child_docs, Doc::line()).append(RcDoc::text(")")))
        .nest(2)
}

fn module_functions_to_doc(module_functions: &ModuleFunctions) -> RcDoc<'static> {
    let funcs: Box<[RcDoc<'static>]> = module_functions
        .iter()
        .map(|f| function_to_doc(f))
        .collect();
    RcDoc::intersperse(funcs, Doc::line())
}

fn function_to_doc(func: &Function) -> RcDoc<'static> {
    let FunctionBuilder {
        instr_seq_arena,
        entry_point,
        ..
    } = &func.builder;

    let name = func.name().to_owned();
    let name_doc = RcDoc::text("$").append(RcDoc::text(name.clone()));
    let export_doc = if func.exported() {
        RcDoc::line()
            .append(RcDoc::text("(export"))
            .append(RcDoc::space())
            .append(RcDoc::text(format!("\"{}\"", name)))
            .append(")")
    } else {
        RcDoc::nil()
    };

    let params = func.params().iter().map(|p| param_to_doc(p));
    let results = func.results().iter().map(|r| result_type_to_doc(r));
    let params_results = params.into_iter().chain(results.into_iter());

    let instr_seq_doc = instr_seq_to_doc(instr_seq_arena, *entry_point);

    RcDoc::text("(func")
        .append(RcDoc::line())
        .append(name_doc)
        .append(export_doc)
        .append(RcDoc::line())
        .append(
            RcDoc::intersperse(params_results, Doc::line())
                .nest(2)
                .group(),
        )
        .group()
        .append(RcDoc::line())
        .append(instr_seq_doc.append(RcDoc::text(")")))
        .nest(2)
}

fn instr_seq_to_doc(arena: &Arena<InstrSeq>, seq_id: InstrSeqId) -> RcDoc<'static> {
    let seq = arena.get(seq_id).unwrap();
    let instrs: Box<[RcDoc<'static>]> = seq
        .0
        .iter()
        .map(|instr| instr_to_doc(arena, instr))
        .collect();

    RcDoc::intersperse(instrs, RcDoc::line())
}

fn instr_to_doc(instr_seq_arena: &Arena<InstrSeq>, instr: &Instr) -> RcDoc<'static> {
    use Instr::*;
    match instr {
        Block(b) => block_to_doc(instr_seq_arena, b),
        Loop(l) => loop_to_doc(instr_seq_arena, l),
        Const(c) => const_to_doc(c),
        Unop(u) => unop_to_doc(u),
        Binop(b) => binop_to_doc(b),
        IfElse(i) => if_else_to_doc(instr_seq_arena, i),
        Drop(_) => RcDoc::text("(drop)"),
        GlobalGet(g) => RcDoc::text("(global.get")
            .append(g.global.clone())
            .append(")"),
        Br(b) => RcDoc::text("(br ")
            .append(b.label.clone())
            .append(")"),
    }
}

fn block_to_doc(instr_seq_arena: &Arena<InstrSeq>, block: &Block) -> RcDoc<'static> {
    let &Block { seq } = block;
    RcDoc::text("(block")
        .append(block_type_to_doc(&UNITYPE.into_block_type_result()))
        .append(RcDoc::line())
        .append(instr_seq_to_doc(instr_seq_arena, seq))
        .append(")")
}

fn loop_to_doc(instr_seq_arena: &Arena<InstrSeq>, l: &Loop) -> RcDoc<'static> {
    let Loop { label, seq } = l;
    RcDoc::text("(loop")
        .append(RcDoc::space())
        .append(RcDoc::text(label.clone()))
        .append(RcDoc::space())
        .append(block_type_to_doc(&UNITYPE.into_block_type_result()))
        .append(RcDoc::line())
        .append(instr_seq_to_doc(instr_seq_arena, *seq))
        .append(")")
        .nest(2)
}

fn const_to_doc(c: &Const) -> RcDoc<'static> {
    let Const { value } = c;
    let ty = match value {
        Value::I32(_) => RcDoc::text("i32.const".to_owned()),
        Value::I64(_) => RcDoc::text("i64.const".to_owned()),
        Value::F32(_) => RcDoc::text("f32.const".to_owned()),
        Value::F64(_) => RcDoc::text("f64.const".to_owned()),
    };

    let value = match value {
        Value::I32(n) => RcDoc::text(format!("{}", n)),
        Value::I64(n) => RcDoc::text(format!("{}", n)),
        Value::F32(n) => RcDoc::text(format!("{}", n)),
        Value::F64(n) => RcDoc::text(format!("{}", n)),
    };

    RcDoc::text("(")
        .append(ty)
        .append(RcDoc::space())
        .append(value)
        .append(RcDoc::text(")"))
}

fn unop_to_doc(unop: &Unop) -> RcDoc<'static> {
    let Unop { op } = unop;
    let op = match op {
        UnaryOp::I32Eqz => "i32.eqz",
        UnaryOp::RefI31 => "ref.i31",
        UnaryOp::I31GetS => "i31.get_s",
        UnaryOp::I31GetU => "i31.get_u",
    };

    RcDoc::text("(")
        .append(RcDoc::text(op))
        .append(RcDoc::text(")"))
}

fn binop_to_doc(binop: &Binop) -> RcDoc<'static> {
    let Binop { op } = binop;
    let op = match op {
        BinaryOp::I32Eq => "i32.eq",
    };

    RcDoc::text("(")
        .append(RcDoc::text(op))
        .append(RcDoc::text(")"))
}

fn if_else_to_doc(instr_seq_arena: &Arena<InstrSeq>, if_else: &IfElse) -> RcDoc<'static> {
    let &IfElse {
        predicate,
        consequent,
        alternative,
    } = if_else;

    let predicate = instr_seq_to_doc(instr_seq_arena, predicate);
    let consequent = instr_seq_to_doc(instr_seq_arena, consequent);
    let alternative = instr_seq_to_doc(instr_seq_arena, alternative);

    RcDoc::text("(if")
        .append(RcDoc::space())
        .append(block_type_to_doc(&UNITYPE.into_block_type_result()))
        .append(RcDoc::line())
        .append(predicate)
        .append(RcDoc::line())
        .append(
            RcDoc::text("(then")
                .append(RcDoc::line().append(consequent).append(")"))
                .nest(2),
        )
        .append(RcDoc::line())
        .append(
            RcDoc::text("(else")
                .append(RcDoc::line().append(alternative).append(")"))
                .nest(2),
        )
        .nest(2)
        .append(")")
}

fn param_to_doc(param: &ParamType) -> RcDoc<'static> {
    let ParamType { name, ty } = param;

    let child_docs: Box<[RcDoc<'static>]> =
        Box::new([RcDoc::text(name.to_owned()), val_type_to_doc(ty)]);

    RcDoc::text("(param")
        .append(RcDoc::space())
        .append(RcDoc::intersperse(child_docs, Doc::line()).nest(1).group())
        .append(")")
}

fn block_type_to_doc(ty: &BlockType) -> RcDoc<'static> {
    match ty {
        BlockType::Result(ty) => result_type_to_doc(ty),
        BlockType::TypeUse(ident) => RcDoc::text(ident.to_owned()),
    }
}

fn result_type_to_doc(ty: &ResultType) -> RcDoc<'static> {
    let ResultType(ty) = ty;

    RcDoc::text("(result")
        .append(RcDoc::space())
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
        NumberType::F64 => "f64",
    })
}

fn ref_type_to_doc(ty: &RefType) -> RcDoc<'static> {
    let RefType {
        nullable,
        heap_type,
    } = ty;

    let name = if *nullable { "(ref" } else { "(ref null" };
    RcDoc::text(name)
        .append(RcDoc::space())
        .append(heap_type_to_doc(heap_type))
        .append(")")
}

fn heap_type_to_doc(ty: &HeapType) -> RcDoc<'static> {
    match ty {
        HeapType::Abstract(ty) => abs_heap_type_to_doc(ty),
        HeapType::Identifier(ident) => RcDoc::text(ident.to_owned()),
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
