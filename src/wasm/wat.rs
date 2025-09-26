use crate::unitype::Unitype;
use crate::wasm::function::{ExportStatus, Function};
use crate::wasm::instr_seq::{InstrSeq, InstrSeqId};
use crate::wasm::intern::IdentifierInterner;
use crate::wasm::module::{Module, ModuleFunctions};
use crate::wasm::types::{
    AbsHeapType, BlockType, GlobalType, HeapType, Mutability, Nullability, NumType, ParamType,
    RefType, ResultType, ValType,
};
use crate::wasm::{
    BinaryOp, Binop, Block, Const, Global, IfElse, Instr, Loop, UnaryOp, Unop, Value,
};
use id_arena::Arena;
use pretty::RcDoc;
use std::borrow::Cow;

type Doc = RcDoc<'static>;

impl Module {
    pub fn to_pretty(&self) -> String {
        let mut w = Vec::new();
        module_to_doc(self).render(80, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

fn module_to_doc(module: &Module) -> Doc {
    let Module {
        interner,
        instr_seq_arena,
        funcs,
        global_arena: globals,
        start,
    } = module;

    let start = match start {
        None => None,
        &Some(start) => Some(funcs.get(start)),
    };

    let child_docs = [
        Some(module_globals_to_doc(interner, instr_seq_arena, globals)),
        Some(module_functions_to_doc(&interner, &instr_seq_arena, funcs)),
        start.map(|f| function_to_doc(&interner, &instr_seq_arena, f)),
    ]
    .into_iter()
    .filter_map(|doc| doc);

    text("(module")
        .append(line())
        .append(RcDoc::intersperse(child_docs, Doc::line()).append(text(")")))
        .nest(2)
}

fn module_globals_to_doc(
    interner: &IdentifierInterner,
    instr_seqs: &Arena<InstrSeq>,
    globals: &Arena<Global>,
) -> Doc {
    let globals: Box<[Doc]> = globals
        .iter()
        .map(|(_, g)| global_to_doc(interner, instr_seqs, g))
        .collect();
    RcDoc::intersperse(globals, Doc::line())
}

/// ```wat
/// (global <id> <global_type>
///     <expr>)
/// ```
fn global_to_doc(interner: &IdentifierInterner, arena: &Arena<InstrSeq>, global: &Global) -> Doc {
    let Global {
        name,
        ty,
        instr_seq,
    } = global;

    let name = interner.get(*name);
    let name = text("\"").append(name.to_owned()).append(text("\""));

    let ty = global_type_to_doc(ty);
    let instr_seq = instr_seq_to_doc(arena, *instr_seq);

    text("(global ")
        .append(space())
        .append(name)
        .append(space())
        .append(ty)
        .append(line())
        .append(instr_seq)
        .append(")")
        .nest(2)
}

fn module_functions_to_doc(
    interner: &IdentifierInterner,
    instr_seq_arena: &Arena<InstrSeq>,
    module_functions: &ModuleFunctions,
) -> Doc {
    let funcs: Box<[Doc]> = module_functions
        .iter()
        .map(|f| function_to_doc(interner, instr_seq_arena, f))
        .collect();
    RcDoc::intersperse(funcs, Doc::line())
}

fn function_to_doc(
    interner: &IdentifierInterner,
    instr_seq_arena: &Arena<InstrSeq>,
    function: &Function,
) -> Doc {
    let Function {
        name,
        exported,
        params,
        results,
        entry_point,
    } = function;

    let name = interner.get(*name);
    let name_doc = text("$").append(text(name.to_owned()));

    let exported = match exported {
        ExportStatus::Exported => line()
            .append(text("(export"))
            .append(space())
            .append(text(format!("\"{}\"", name)))
            .append(")"),
        ExportStatus::NotExported => RcDoc::nil(),
    };

    let params = params.iter().map(|p| param_to_doc(p));
    let results = results.iter().map(|r| result_type_to_doc(r));
    let params_results = params.into_iter().chain(results.into_iter());

    let instr_seq_doc = instr_seq_to_doc(instr_seq_arena, *entry_point);

    text("(func")
        .append(line())
        .append(name_doc)
        .append(exported)
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

fn instr_seq_to_doc(arena: &Arena<InstrSeq>, seq_id: InstrSeqId) -> Doc {
    let seq = arena.get(seq_id).unwrap();
    let instrs: Box<[Doc]> = seq
        .0
        .iter()
        .map(|instr| instr_to_doc(arena, instr))
        .collect();

    RcDoc::intersperse(instrs, line())
}

fn instr_to_doc(instr_seq_arena: &Arena<InstrSeq>, instr: &Instr) -> Doc {
    use Instr::*;
    match instr {
        Block(b) => block_to_doc(instr_seq_arena, b),
        Loop(l) => loop_to_doc(instr_seq_arena, l),
        Const(c) => const_to_doc(c),
        Unop(u) => unop_to_doc(u),
        Binop(b) => binop_to_doc(b),
        IfElse(i) => if_else_to_doc(instr_seq_arena, i),
        Drop(_) => text("(drop)"),
        GlobalGet(g) => text("(global.get")
            .append(space())
            .append(g.global.clone())
            .append(")"),
        Br(b) => text("(br ").append(b.label.clone()).append(")"),
    }
}

fn block_to_doc(instr_seq_arena: &Arena<InstrSeq>, block: &Block) -> Doc {
    let &Block { seq } = block;
    text("(block")
        .append(block_type_to_doc(
            &Unitype::UNITYPE.into_block_type_result(),
        ))
        .append(line())
        .append(instr_seq_to_doc(instr_seq_arena, seq))
        .append(")")
}

fn loop_to_doc(instr_seq_arena: &Arena<InstrSeq>, l: &Loop) -> Doc {
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

fn const_to_doc(c: &Const) -> Doc {
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

fn unop_to_doc(unop: &Unop) -> Doc {
    let Unop { op } = unop;
    let op = match op {
        UnaryOp::I32Eqz => "i32.eqz",
        UnaryOp::RefI31 => "ref.i31",
        UnaryOp::I31GetS => "i31.get_s",
        UnaryOp::I31GetU => "i31.get_u",
    };

    text("(").append(text(op)).append(text(")"))
}

fn binop_to_doc(binop: &Binop) -> Doc {
    let Binop { op } = binop;
    let op = match op {
        BinaryOp::I32Eq => "i32.eq",
    };

    text("(").append(text(op)).append(text(")"))
}

fn if_else_to_doc(instr_seq_arena: &Arena<InstrSeq>, if_else: &IfElse) -> Doc {
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

fn param_to_doc(param: &ParamType) -> Doc {
    let ParamType { name, ty } = param;

    let child_docs: Box<[Doc]> = Box::new([text(name.to_owned()), val_type_to_doc(ty)]);

    text("(param")
        .append(space())
        .append(RcDoc::intersperse(child_docs, Doc::line()).nest(1).group())
        .append(")")
}

/// ```wat
/// <global_type> = <valtype>
///               | (mut <valtype>)
/// ```
fn global_type_to_doc(ty: &GlobalType) -> Doc {
    let GlobalType { mutable, val_type } = ty;
    let val_type = val_type_to_doc(val_type);
    match mutable {
        Mutability::Mut => text("(mut ").append(space()).append(val_type).append(")"),
        Mutability::Const => val_type,
    }
}

fn block_type_to_doc(ty: &BlockType) -> Doc {
    match ty {
        BlockType::Result(ty) => result_type_to_doc(ty),
        BlockType::TypeUse(ident) => text(ident.to_owned()),
    }
}

fn result_type_to_doc(ty: &ResultType) -> Doc {
    let ResultType(ty) = ty;

    text("(result")
        .append(space())
        .append(val_type_to_doc(ty))
        .append(")")
}

fn val_type_to_doc(ty: &ValType) -> Doc {
    match ty {
        ValType::NumType(ty) => num_type_to_doc(ty),
        ValType::Ref(ty) => ref_type_to_doc(ty),
    }
}

fn num_type_to_doc(ty: &NumType) -> Doc {
    text(match ty {
        NumType::I32 => "i32",
        NumType::I64 => "i64",
        NumType::F32 => "f32",
        NumType::F64 => "f64",
    })
}

fn ref_type_to_doc(ty: &RefType) -> Doc {
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

fn heap_type_to_doc(ty: &HeapType) -> Doc {
    match ty {
        HeapType::Abstract(ty) => abs_heap_type_to_doc(ty),
        HeapType::Identifier(ident) => text(ident.to_owned()),
    }
}

fn abs_heap_type_to_doc(ty: &AbsHeapType) -> Doc {
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

fn text<S: Into<Cow<'static, str>>>(str: S) -> Doc {
    RcDoc::text(str)
}

fn space() -> Doc {
    RcDoc::space()
}

fn line() -> Doc {
    RcDoc::line()
}
