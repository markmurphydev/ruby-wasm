//! Pretty-print a [Module] as WAT.
//!
//! Pretty-printing tips:
//! - To indent the first line of a nested block,
//!     make sure it's the first element of the nested append.

use crate::unitype::Unitype;
use crate::wasm::function::{ExportStatus, Function, Local};
use crate::wasm::instr_seq::{InstrSeq, InstrSeqId};
use crate::wasm::intern::IdentifierInterner;
use crate::wasm::module::{Module, ModuleFunctions};
use crate::wasm::types::{
    AbsHeapType, ArrayType, BlockType, CompType, FieldType, FuncType, GlobalType, HeapType,
    Mutability, Nullability, NumType, PackType, ParamType, RefType, ResultType, StorageType,
    StructType, SubType, ValType,
};
use crate::wasm::{
    BinaryOp, Binop, Block, Const, Finality, Global, IfElse, Instr, Loop, RefCast, RefTest,
    TypeDef, UnaryOp, Unop, Value,
};
use id_arena::Arena;
use pretty::RcDoc;
use std::any::Any;
use std::borrow::Cow;

type Doc = RcDoc<'static>;

const INDENT: isize = 2;

const WAT_CORE: &str = include_str!("../../core_generated.wat");

impl Module {
    pub fn to_pretty(&self) -> String {
        let mut w = Vec::new();
        module_to_doc(self).render(80, &mut w).unwrap();
        String::from_utf8(w).unwrap()
    }
}

/// ```wat
/// (module
///     <globals>
///     <functions>)
/// ```
fn module_to_doc(module: &Module) -> Doc {
    let Module {
        interner,
        instr_seq_arena,
        funcs,
        type_def_arena: type_defs,
        global_arena: globals,
        start,
    } = module;

    let start = start.map(|start| {
        let start = funcs.get(start);
        let start_identifier = interner.get(start.name);
        text(format!("(start ${})", start_identifier))
    });

    let module_fields = [
        Some(module_type_defs_to_doc(interner, type_defs)),
        Some(module_globals_to_doc(interner, instr_seq_arena, globals)),
        Some(module_functions_to_doc(&interner, &instr_seq_arena, funcs)),
        start
    ]
    .into_iter()
    .filter_map(|doc| doc);

    intersperse(module_fields, hardline())
}

/// ```wat
/// <global>*
/// ```
fn module_globals_to_doc(
    interner: &IdentifierInterner,
    instr_seqs: &Arena<InstrSeq>,
    globals: &Arena<Global>,
) -> Doc {
    let globals: Box<[Doc]> = globals
        .iter()
        .map(|(_, g)| global_to_doc(interner, instr_seqs, g))
        .collect();
    intersperse(globals, hardline())
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
    let name = format!("${}", name);

    let ty = global_type_to_doc(ty);
    let instr_seq = instr_seq_to_doc(arena, *instr_seq);

    text("(global")
        .append(space())
        .append(name)
        .append(line())
        .append(ty)
        .append(hardline())
        .append(instr_seq)
        .append(")")
        .nest(INDENT)
        .group()
}

/// ```wat
/// <type>*
/// ```
fn module_type_defs_to_doc(interner: &IdentifierInterner, types: &Arena<TypeDef>) -> Doc {
    let type_defs: Box<[Doc]> = types
        .iter()
        .map(|(_, td)| type_def_to_doc(interner, td))
        .collect();
    text("(rec")
        .append(line())
        .append(intersperse(type_defs, hardline()))
        .append(")")
        .nest(INDENT)
        .group()
}

/// ```wat
/// (type id <type>)
/// ```
fn type_def_to_doc(interner: &IdentifierInterner, type_def: &TypeDef) -> Doc {
    let TypeDef { name, ty } = type_def;
    let name = interner.get(*name);
    let name = format!("${}", name);

    let ty = sub_type_to_doc(ty);

    text("(type")
        .append(space())
        .append(name)
        .append(line())
        .append(ty)
        .append(")")
        .nest(INDENT)
        .group()
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
    intersperse(funcs, hardline())
}

/// ```wat
/// func ::=
/// (func <id> <export> <param>* <result>+
///     <instr_seq>)
///
/// export ::=
/// (export "<name>")
///
/// result ::=
/// (result <val_type>)
/// ```
fn function_to_doc(
    interner: &IdentifierInterner,
    instr_seq_arena: &Arena<InstrSeq>,
    function: &Function,
) -> Doc {
    let Function {
        name,
        exported,
        type_use,
        params,
        results,
        entry_point,
        locals,
    } = function;

    let name = interner.get(*name);
    let name_doc = format!("${}", name);

    let exported = match exported {
        ExportStatus::Exported => line().append(format!("(export \"{}\")", name)),
        ExportStatus::NotExported => nil(),
    };

    let type_use = match type_use {
        None => nil(),
        Some(ident) => line().append(format!("(type ${})", ident)),
    };

    let params = if params.is_empty() {
        nil()
    } else {
        let params = params.iter().map(param_type_to_doc);
        line().append(intersperse(params, line()).group())
    };
    let results = {
        let results = results.iter().map(result_type_to_doc);
        line().append(intersperse(results, line()).group())
    };
    let locals = if locals.is_empty() {
        nil()
    } else {
        let locals = locals.iter().map(local_to_doc);
        line().append(intersperse(locals, line()).group())
    };

    let instr_seq_doc = instr_seq_to_doc(instr_seq_arena, *entry_point);

    text("(func")
        .append(line())
        .append(name_doc)
        .append(exported)
        .append(type_use)
        .append(params)
        .append(results)
        .append(locals)
        .append(hardline())
        .append(instr_seq_doc)
        .append(")")
        .nest(INDENT)
        .group()
}

fn local_to_doc(local: &Local) -> Doc {
    let Local { identifier, ty } = local;
    let ty = val_type_to_doc(&ty);
    text(format!("(local ${}", identifier))
        .append(line())
        .append(ty)
        .append(")")
        .nest(INDENT)
        .group()
}

fn instr_seq_to_doc(arena: &Arena<InstrSeq>, seq_id: InstrSeqId) -> Doc {
    let seq = arena.get(seq_id).unwrap();
    let instrs: Box<[Doc]> = seq
        .0
        .iter()
        .map(|instr| instr_to_doc(arena, instr))
        .collect();

    intersperse(instrs, line())
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
        GlobalGet(global) => text(format!("(global.get ${})", global.name.clone())),
        Br(br) => text(format!("(br ${})", br.label.clone())),
        Call(call) => text(format!("(call ${})", call.func.clone())),
        LocalGet(get) => text(format!("(local.get ${})", get.name.clone())),
        BrIf(br_if) => text(format!("(br_if ${})", br_if.block.clone())),
        RefTest(r) => ref_test_to_doc(r),
        RefCast(c) => ref_cast_to_doc(c),
        ArrayNewFixed(arr) => text(format!(
            "(array.new_fixed ${} {})",
            arr.type_name, arr.length
        )),
        RefNull(r) => text(format!("(ref.null ${})", r.type_name)),
        StructNew(s) => text(format!("(struct.new ${})", s.type_name)),
        RefFunc(r) => text(format!("(ref.func ${})", r.func_name)),
        StructGet(sg) => text(format!("(struct.get ${} ${})", sg.type_name, sg.field_name)),
        StructSet(ss) => text(format!("(struct.set ${} ${})", ss.type_name, ss.field_name)),
        LocalSet(ls) => text(format!("(local.set ${})", ls.name)),
        Unreachable(_) => text("(unreachable)"),
        Return(_) => text("(return)"),
        ArrayGetU(agu) => text(format!("(array.get_u ${})", agu.type_name)),
        ArrayGet(ag) => text(format!("(array.get ${})", ag.type_name)),
        CallRef(cr) => text(format!("(call_ref ${})", cr.type_name))
    }
}

/// ```wat
/// (block <label> <block_type>
///     <instr>*)
/// ```
fn block_to_doc(instr_seq_arena: &Arena<InstrSeq>, block: &Block) -> Doc {
    let &Block { seq } = block;
    let block_type = block_type_to_doc(&Unitype::UNITYPE.into_block_type_result());
    let instr_seq = instr_seq_to_doc(instr_seq_arena, seq);
    text("(block")
        .append(
            line().append(block_type).group(), // insert label here
        )
        .append(hardline())
        .append(instr_seq)
        .append(")")
        .nest(INDENT)
        .group()
}

/// ```wat
/// (loop <label> <block_type>
///     <instr>*)
/// ```
fn loop_to_doc(instr_seq_arena: &Arena<InstrSeq>, l: &Loop) -> Doc {
    let Loop { label, seq } = l;
    let label = text(format!("${}", label));
    let block_type = block_type_to_doc(&Unitype::UNITYPE.into_block_type_result());
    let instr_seq = instr_seq_to_doc(instr_seq_arena, *seq);
    text("(loop")
        .append(
            line()
                .append(label)
                .append(line())
                .append(block_type)
                .group(),
        )
        .append(hardline())
        .append(instr_seq)
        .append(")")
        .nest(INDENT)
        .group()
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
        .append(")")
        .group()
}

fn unop_to_doc(unop: &Unop) -> Doc {
    let Unop { op } = unop;
    let op = match op {
        UnaryOp::I32Eqz => "i32.eqz",
        UnaryOp::RefI31 => "ref.i31",
        UnaryOp::I31GetS => "i31.get_s",
        UnaryOp::I31GetU => "i31.get_u",
        UnaryOp::ArrayLen => "array.len",
        UnaryOp::RefAsNonNull => "ref.as_non_null",
    };

    text(format!("({})", op))
}

fn binop_to_doc(binop: &Binop) -> Doc {
    let Binop { op } = binop;
    let op = match op {
        BinaryOp::I32Eq => "i32.eq",
        BinaryOp::I32Add => "i32.add",
    };

    text(format!("({})", op))
}

/// ```wat
/// (if <label> <block_type>
///     <instr>*
///     (then <instr>+)
///     (else <instr>+)?)
/// ```
fn if_else_to_doc(instr_seq_arena: &Arena<InstrSeq>, if_else: &IfElse) -> Doc {
    let IfElse {
        ty,
        predicate,
        consequent,
        alternative,
    } = if_else;

    let ty = match ty {
        None => nil(),
        Some(ty) => line().append(block_type_to_doc(ty)),
    };
    let predicate = instr_seq_to_doc(instr_seq_arena, *predicate);
    let consequent = {
        let seq = instr_seq_to_doc(instr_seq_arena, *consequent);
        text("(then")
            .append(hardline())
            .append(seq)
            .append(")")
            .nest(INDENT)
    };
    let alternative = {
        let seq = instr_seq_to_doc(instr_seq_arena, *alternative);
        text("(else")
            .append(hardline())
            .append(seq)
            .append(")")
            .nest(INDENT)
    };

    text("(if")
        .append(ty)
        .append(hardline())
        .append(predicate)
        .append(hardline())
        .append(consequent)
        .append(hardline())
        .append(alternative)
        .append(")")
        .nest(INDENT)
}

/// ```wat
/// (ref.test <ty>)
/// ```
fn ref_test_to_doc(ref_test: &RefTest) -> Doc {
    let RefTest { ty } = ref_test;
    let ty = ref_type_to_doc(ty);
    text("(ref.test")
        .append(line())
        .append(ty)
        .append(")")
        .nest(INDENT)
        .group()
}

/// ```wat
/// (ref.cast <ty>)
/// ```
fn ref_cast_to_doc(ref_cast: &RefCast) -> Doc {
    let RefCast { result_ty } = ref_cast;
    let result_ty = ref_type_to_doc(result_ty);
    text("(ref.cast")
        .append(line())
        .append(result_ty)
        .append(")")
        .nest(INDENT)
        .group()
}

/// ```wat
/// (param <id> <val_type>)
/// ```
fn param_type_to_doc(param: &ParamType) -> Doc {
    let ParamType { name, ty } = param;

    let name = format!("${}", name);
    let ty = val_type_to_doc(ty);

    text("(param")
        .append(line())
        .append(name)
        .append(line())
        .append(ty)
        .append(")")
        .nest(INDENT)
        .group()
}

/// ```wat
/// (result <val_type>)
/// ```
fn result_type_to_doc(ty: &ResultType) -> Doc {
    let ResultType(ty) = ty;

    text("(result")
        .append(line())
        .append(val_type_to_doc(ty))
        .append(")")
        .nest(INDENT)
        .group()
}

/// ```wat
/// <global_type> = <valtype>
///               | (mut <valtype>)
/// ```
fn global_type_to_doc(ty: &GlobalType) -> Doc {
    let GlobalType { mutable, val_type } = ty;
    let val_type = val_type_to_doc(val_type);
    match mutable {
        Mutability::Mut => text("(mut")
            .append(line())
            .append(val_type)
            .append(")")
            .nest(INDENT)
            .group(),
        Mutability::Const => val_type,
    }
}

fn block_type_to_doc(ty: &BlockType) -> Doc {
    match ty {
        BlockType::Result(ty) => result_type_to_doc(ty),
        BlockType::TypeUse(ident) => text(ident.to_owned()),
    }
}

fn val_type_to_doc(ty: &ValType) -> Doc {
    match ty {
        ValType::Num(ty) => num_type_to_doc(ty),
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

/// ```wat
/// (ref null? <heap_type>)
/// ```
fn ref_type_to_doc(ty: &RefType) -> Doc {
    let RefType {
        nullable,
        heap_type,
    } = ty;

    let nullability = match nullable {
        Nullability::Nullable => space().append(text("null")),
        Nullability::NonNullable => nil(),
    };

    text("(ref")
        .append(nullability)
        .append(line())
        .append(heap_type_to_doc(heap_type))
        .append(")")
        .nest(INDENT)
        .group()
}

fn heap_type_to_doc(ty: &HeapType) -> Doc {
    match ty {
        HeapType::Abstract(ty) => abs_heap_type_to_doc(ty),
        HeapType::Identifier(ident) => text(format!("${}", ident)),
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

/// `(sub final? <supertype>* <comp_type>
fn sub_type_to_doc(ty: &SubType) -> Doc {
    let SubType {
        is_final,
        supertypes,
        comp_type,
    } = ty;
    let is_final = match is_final {
        Finality::Final => line().append(text("final")),
        Finality::NotFinal => nil(),
    };
    let supertypes: Vec<Doc> = supertypes
        .into_iter()
        .map(|s| format!("${}", s))
        .map(text)
        .collect();
    let supertypes = if supertypes.is_empty() {
        nil()
    } else {
        line().append(intersperse(supertypes, line()))
    };
    let comp_type = comp_type_to_doc(comp_type);

    text("(sub")
        .append(is_final)
        .append(supertypes)
        .append(line())
        .append(comp_type)
        .append(text(")"))
        .nest(INDENT)
        .group()
}

fn comp_type_to_doc(ty: &CompType) -> Doc {
    match ty {
        CompType::Struct(ty) => struct_type_to_doc(ty),
        CompType::Array(ty) => array_type_to_doc(ty),
        CompType::Func(ty) => func_type_to_doc(ty),
    }
}

/// ```wat
/// struct_type ::= (struct <field>*)
///
/// field ::= (field <id> <field_type>)
/// ```
fn struct_type_to_doc(ty: &StructType) -> Doc {
    let StructType { fields } = ty;

    let fields: Vec<Doc> = fields
        .iter()
        .map(|(name, ty)| {
            text("(field")
                .append(space())
                .append(text(format!("${}", name)))
                .append(line())
                .append(field_type_to_doc(ty))
                .append(")")
                .nest(INDENT)
                .group()
        })
        .collect();
    let fields = intersperse(fields, line());

    text("(struct")
        .append(line())
        .append(fields)
        .append(")")
        .nest(INDENT)
        .group()
}

/// ```wat
/// (array <field_type>*)
/// ```
fn array_type_to_doc(ty: &ArrayType) -> Doc {
    let ArrayType { field } = ty;
    let field = field_type_to_doc(field);

    text("(array")
        .append(line())
        .append(field)
        .append(")")
        .nest(INDENT)
        .group()
}

/// ```wat
/// (func <param>* <result>*)
/// ```
fn func_type_to_doc(ty: &FuncType) -> Doc {
    let FuncType { params, results } = ty;
    let params: Vec<Doc> = params.iter().map(|ty| param_type_to_doc(ty)).collect();
    let params = intersperse(params, line());

    let results: Vec<Doc> = results.iter().map(|ty| result_type_to_doc(ty)).collect();
    let results = intersperse(results, line());

    text("(func")
        .append(line())
        .append(params)
        .append(line())
        .append(results)
        .append(")")
        .nest(INDENT)
        .group()
}

/// ```wat
/// (mut? <storage_type>)
/// ```
fn field_type_to_doc(ty: &FieldType) -> Doc {
    let FieldType { mutability, ty } = ty;
    let ty = storage_type_to_doc(ty);

    match mutability {
        Mutability::Const => ty,
        Mutability::Mut => text("(mut")
            .append(line())
            .append(ty)
            .append(")")
            .nest(INDENT)
            .group(),
    }
}

fn storage_type_to_doc(ty: &StorageType) -> Doc {
    match ty {
        StorageType::Val(ty) => val_type_to_doc(ty),
        StorageType::Pack(ty) => pack_type_to_doc(ty),
    }
}

fn pack_type_to_doc(ty: &PackType) -> Doc {
    match ty {
        PackType::I8 => text("i8"),
        PackType::I16 => text("i16"),
    }
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

fn hardline() -> Doc {
    RcDoc::hardline()
}

fn intersperse<D: IntoIterator<Item = Doc>>(docs: D, separator: Doc) -> Doc {
    RcDoc::intersperse(docs, separator)
}

fn nil() -> Doc {
    RcDoc::nil()
}
