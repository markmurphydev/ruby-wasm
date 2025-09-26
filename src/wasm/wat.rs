use crate::FunctionBuilder;
use crate::unitype::Unitype;
use crate::wasm::function::{ExportStatus, Function, InstrSeq, InstrSeqId};
use crate::wasm::module::{Module, ModuleFunctions, ModuleGlobals};
use crate::wasm::types::{
    AbsHeapType, BlockType, GlobalType, HeapType, Mutability, Nullability, NumType, ParamType,
    RefType, ResultType, ValType,
};
use crate::wasm::{
    BinaryOp, Binop, Block, Const, Global, IfElse, Instr, Loop, UnaryOp, Unop, Value,
};
use id_arena::Arena;
use pretty::{Doc, RcDoc};
use std::borrow::Cow;

impl Module {
    pub fn to_pretty(&self) -> String {
        let mut w = Vec::new();
        module_to_doc(self).render(80, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

fn module_to_doc(module: &Module) -> RcDoc<'static> {
    let Module {
        interner: _interner,
        instr_seq_arena,
        funcs,
        globals,
        start,
    } = module;

    let start = match start {
        None => None,
        &Some(start) => Some(funcs.get(start)),
    };

    let child_docs = [
        Some(module_globals_to_doc(instr_seq_arena, globals)),
        Some(module_functions_to_doc(funcs)),
        start.map(|f| function_to_doc(f)),
    ]
    .into_iter()
    .filter_map(|doc| doc);

    text("(module")
        .append(line())
        .append(RcDoc::intersperse(child_docs, Doc::line()).append(text(")")))
        .nest(2)
}

fn module_globals_to_doc(
    arena: &Arena<InstrSeq>,
    module_globals: &ModuleGlobals,
) -> RcDoc<'static> {
    let globals: Box<[RcDoc<'static>]> = module_globals
        .iter()
        .map(|g| global_to_doc(arena, g))
        .collect();
    RcDoc::intersperse(globals, Doc::line())
}

/// ```wat
/// (global <id> <global_type>
///     <expr>)
/// ```
fn global_to_doc(arena: &Arena<InstrSeq>, global: &Global) -> RcDoc<'static> {
    let Global {
        name,
        ty,
        instr_seq,
    } = global;

    let ty = global_type_to_doc(ty);
    let instr_seq = instr_seq_to_doc(arena, *instr_seq);

    text("(global ")
        .append(space())
        .append(text(name.clone()))
        .append(line())
        .append(instr_seq)
        .append(")")
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
    let name_doc = text("$").append(text(name.clone()));
    let export_doc = match func.exported() {
        ExportStatus::Exported =>
        line()
            .append(text("(export"))
            .append(space())
            .append(text(format!("\"{}\"", name)))
            .append(")"),
        ExportStatus::NotExported =>
            RcDoc::nil()
    };

    let params = func.params().iter().map(|p| param_to_doc(p));
    let results = func.results().iter().map(|r| result_type_to_doc(r));
    let params_results = params.into_iter().chain(results.into_iter());

    let instr_seq_doc = instr_seq_to_doc(instr_seq_arena, *entry_point);

    text("(func")
        .append(line())
        .append(name_doc)
        .append(export_doc)
        .append(line())
        .append(
            RcDoc::intersperse(params_results, Doc::line())
                .nest(2)
                .group(),
        )
        .group()
        .append(line())
        .append(instr_seq_doc.append(text(")")))
        .nest(2)
}

fn instr_seq_to_doc(arena: &Arena<InstrSeq>, seq_id: InstrSeqId) -> RcDoc<'static> {
    let seq = arena.get(seq_id).unwrap();
    let instrs: Box<[RcDoc<'static>]> = seq
        .0
        .iter()
        .map(|instr| instr_to_doc(arena, instr))
        .collect();

    RcDoc::intersperse(instrs, line())
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
        Drop(_) => text("(drop)"),
        GlobalGet(g) => text("(global.get").append(g.global.clone()).append(")"),
        Br(b) => text("(br ").append(b.label.clone()).append(")"),
    }
}

fn block_to_doc(instr_seq_arena: &Arena<InstrSeq>, block: &Block) -> RcDoc<'static> {
    let &Block { seq } = block;
    text("(block")
        .append(block_type_to_doc(
            &Unitype::UNITYPE.into_block_type_result(),
        ))
        .append(line())
        .append(instr_seq_to_doc(instr_seq_arena, seq))
        .append(")")
}

fn loop_to_doc(instr_seq_arena: &Arena<InstrSeq>, l: &Loop) -> RcDoc<'static> {
    let Loop { label, seq } = l;
    text("(loop")
        .append(space())
        .append(text(label.clone()))
        .append(space())
        .append(block_type_to_doc(
            &Unitype::UNITYPE.into_block_type_result(),
        ))
        .append(line())
        .append(instr_seq_to_doc(instr_seq_arena, *seq))
        .append(")")
        .nest(2)
}

fn const_to_doc(c: &Const) -> RcDoc<'static> {
    let Const { value } = c;
    let ty = match value {
        Value::I32(_) => text("i32.const".to_owned()),
        Value::I64(_) => text("i64.const".to_owned()),
        Value::F32(_) => text("f32.const".to_owned()),
        Value::F64(_) => text("f64.const".to_owned()),
    };

    let value = match value {
        Value::I32(n) => text(format!("{}", n)),
        Value::I64(n) => text(format!("{}", n)),
        Value::F32(n) => text(format!("{}", n)),
        Value::F64(n) => text(format!("{}", n)),
    };

    text("(")
        .append(ty)
        .append(space())
        .append(value)
        .append(text(")"))
}

fn unop_to_doc(unop: &Unop) -> RcDoc<'static> {
    let Unop { op } = unop;
    let op = match op {
        UnaryOp::I32Eqz => "i32.eqz",
        UnaryOp::RefI31 => "ref.i31",
        UnaryOp::I31GetS => "i31.get_s",
        UnaryOp::I31GetU => "i31.get_u",
    };

    text("(").append(text(op)).append(text(")"))
}

fn binop_to_doc(binop: &Binop) -> RcDoc<'static> {
    let Binop { op } = binop;
    let op = match op {
        BinaryOp::I32Eq => "i32.eq",
    };

    text("(").append(text(op)).append(text(")"))
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

    text("(if")
        .append(space())
        .append(block_type_to_doc(
            &Unitype::UNITYPE.into_block_type_result(),
        ))
        .append(line())
        .append(predicate)
        .append(line())
        .append(
            text("(then")
                .append(line().append(consequent).append(")"))
                .nest(2),
        )
        .append(line())
        .append(
            text("(else")
                .append(line().append(alternative).append(")"))
                .nest(2),
        )
        .nest(2)
        .append(")")
}

fn param_to_doc(param: &ParamType) -> RcDoc<'static> {
    let ParamType { name, ty } = param;

    let child_docs: Box<[RcDoc<'static>]> = Box::new([text(name.to_owned()), val_type_to_doc(ty)]);

    text("(param")
        .append(space())
        .append(RcDoc::intersperse(child_docs, Doc::line()).nest(1).group())
        .append(")")
}

/// ```wat
/// <global_type> = <valtype>
///               | (mut <valtype>)
/// ```
fn global_type_to_doc(ty: &GlobalType) -> RcDoc<'static> {
    let GlobalType { mutable, val_type } = ty;
    let val_type = val_type_to_doc(val_type);
    match mutable {
        Mutability::Mut => text("(mut ").append(space()).append(val_type).append(")"),
        Mutability::Const => val_type,
    }
}

fn block_type_to_doc(ty: &BlockType) -> RcDoc<'static> {
    match ty {
        BlockType::Result(ty) => result_type_to_doc(ty),
        BlockType::TypeUse(ident) => text(ident.to_owned()),
    }
}

fn result_type_to_doc(ty: &ResultType) -> RcDoc<'static> {
    let ResultType(ty) = ty;

    text("(result")
        .append(space())
        .append(val_type_to_doc(ty))
        .append(")")
}

fn val_type_to_doc(ty: &ValType) -> RcDoc<'static> {
    match ty {
        ValType::NumType(ty) => num_type_to_doc(ty),
        ValType::Ref(ty) => ref_type_to_doc(ty),
    }
}

fn num_type_to_doc(ty: &NumType) -> RcDoc<'static> {
    text(match ty {
        NumType::I32 => "i32",
        NumType::I64 => "i64",
        NumType::F32 => "f32",
        NumType::F64 => "f64",
    })
}

fn ref_type_to_doc(ty: &RefType) -> RcDoc<'static> {
    let RefType {
        nullable,
        heap_type,
    } = ty;

    let name = match nullable {
        Nullability::Nullable => "(ref null",
        Nullability::NonNullable => "(ref",
    };
    text(name)
        .append(space())
        .append(heap_type_to_doc(heap_type))
        .append(")")
}

fn heap_type_to_doc(ty: &HeapType) -> RcDoc<'static> {
    match ty {
        HeapType::Abstract(ty) => abs_heap_type_to_doc(ty),
        HeapType::Identifier(ident) => text(ident.to_owned()),
    }
}

fn abs_heap_type_to_doc(ty: &AbsHeapType) -> RcDoc<'static> {
    text(match ty {
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

fn text<S: Into<Cow<'static, str>>>(str: S) -> RcDoc<'static> {
    RcDoc::text(str)
}

fn space() -> RcDoc<'static> {
    RcDoc::space()
}

fn line() -> RcDoc<'static> {
    RcDoc::line()
}
