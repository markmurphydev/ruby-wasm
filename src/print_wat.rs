//! Pretty-print a [Module] as WAT.
//!
//! Pretty-printing tips:
//! - To indent the first line of a nested block,
//!     make sure it's the first element of the nested append.

use pretty::RcDoc;
use std::borrow::Cow;
use wat_defs::func::{Exported, Func, Imported, Local, Param};
use wat_defs::global::Global;
use wat_defs::instr::UnfoldedInstr::{
    I32WrapI64, I64Add, I64Xor, Nop, RefAsNonNull, RefI31, Return,
};
use wat_defs::instr::{Instr, UnfoldedInstr};
use wat_defs::module::{Module, TypeDef};
use wat_defs::ty::{
    AbsHeapType, ArrayType, BlockType, CompType, Field, FieldType, Final, FuncType, GlobalType,
    HeapType, Mutable, Nullable, NumType, PackType, RefType, StorageType, StructType, SubType,
    ValType,
};

type Doc = RcDoc<'static>;

const INDENT: isize = 2;

pub fn module_to_pretty(module: &Module) -> String {
    let mut w = Vec::new();
    module_to_doc(module).render(80, &mut w).unwrap();
    String::from_utf8(w).unwrap()
}

/// ```wat
/// (module
///     <globals>
///     <functions>)
/// ```
fn module_to_doc(module: &Module) -> Doc {
    let Module {
        types,
        globals,
        funcs,
        start_fn,
    } = module;

    let start_fn = start_fn
        .clone()
        .map(|name| text(format!("(start ${})", name)));

    let module_fields = [
        Some(module_type_defs_to_doc(types)),
        Some(module_functions_to_doc(funcs)),
        Some(module_globals_to_doc(globals)),
        start_fn,
    ]
    .into_iter()
    .filter_map(|doc| doc);

    intersperse(module_fields, hardline())
}

/// ```wat
/// <global>*
/// ```
fn module_globals_to_doc(globals: &Vec<Global>) -> Doc {
    let globals: Box<[Doc]> = globals.iter().map(global_to_doc).collect();
    intersperse(globals, hardline())
}

/// ```wat
/// (global <id> <global_type>
///     <expr>)
/// ```
fn global_to_doc(global: &Global) -> Doc {
    let Global {
        name,
        ty,
        instr_seq,
    } = global;

    let name = format!("${}", name);

    let ty = global_type_to_doc(ty);
    let instr_seq = instr_seq_to_doc(instr_seq);

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
fn module_type_defs_to_doc(type_defs: &Vec<TypeDef>) -> Doc {
    let type_defs = type_defs.iter().map(type_def_to_doc);
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
fn type_def_to_doc(type_def: &TypeDef) -> Doc {
    let TypeDef { name, ty } = type_def;
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

fn module_functions_to_doc(funcs: &Vec<Func>) -> Doc {
    let funcs = funcs.iter().map(function_to_doc);
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
fn function_to_doc(func: &Func) -> Doc {
    let Func {
        name,
        imported,
        exported,
        type_use,
        params,
        results,
        locals,
        instrs,
    } = func;

    let name_doc = format!("${}", name);

    let imported = match imported {
        Imported::NotImported => nil(),
        Imported::Imported(module, name) => {
            line().append(format!("(import \"{}\" \"{}\")", module, name))
        }
    };

    let exported = match exported {
        Exported::NotExported => nil(),
        Exported::Exported(name) => line().append(format!("(export \"{}\")", name)),
    };

    let type_use = match type_use {
        None => nil(),
        Some(ident) => line().append(format!("(type ${})", ident)),
    };

    let params = if params.is_empty() {
        nil()
    } else {
        let params = params.iter().map(param_to_doc);
        line().append(intersperse(params, line()).group())
    };
    let results = {
        let results = results.iter().map(result_to_doc);
        line().append(intersperse(results, line()).group())
    };
    let locals = if locals.is_empty() {
        nil()
    } else {
        let locals = locals.iter().map(local_to_doc);
        line().append(intersperse(locals, line()).group())
    };

    let instrs = instr_seq_to_doc(instrs);

    text("(func")
        .append(line())
        .append(name_doc)
        .append(imported)
        .append(exported)
        .append(type_use)
        .append(params)
        .append(results)
        .append(locals)
        .append(hardline())
        .append(instrs)
        .append(")")
        .nest(INDENT)
        .group()
}

fn local_to_doc(local: &Local) -> Doc {
    let Local { name, ty } = local;
    let ty = val_type_to_doc(&ty);
    text(format!("(local ${}", name))
        .append(line())
        .append(ty)
        .append(")")
        .nest(INDENT)
        .group()
}

fn instr_seq_to_doc(instrs: &Vec<Instr>) -> Doc {
    let instrs = instrs.iter().map(instr_to_doc);
    intersperse(instrs, line())
}

fn instrs_to_doc(instrs: &Vec<Instr>) -> Doc {
    let instrs = instrs.iter().map(instr_to_doc);
    intersperse(instrs, hardline())
}

fn instr_to_doc(instr: &Instr) -> Doc {
    let Instr {
        unfolded_instr,
        folded_instrs,
    } = instr;

    if matches!(unfolded_instr, UnfoldedInstr::If { .. }) {
        return if_instr_to_doc(instr);
    }

    let unfolded_instr = unfolded_instr_to_doc(unfolded_instr);
    let folded_instrs = if folded_instrs.is_empty() {
        nil()
    } else {
        hardline().append(instrs_to_doc(folded_instrs))
    };

    text("(")
        .append(unfolded_instr)
        .append(folded_instrs)
        .append(")")
        .nest(INDENT)
        .group()
}

fn unfolded_instr_to_doc(instr: &UnfoldedInstr) -> Doc {
    use UnfoldedInstr::*;
    match instr {
        Nop => text("nop"),
        Drop => text("drop"),
        Const { ty, val } => const_to_doc(ty, *val),
        I32Eqz => text("i32.eqz"),
        I32Eq => text("i32.eq"),
        I32LtS => text("i32.lt_s"),
        I32LtU => text("i32.lt_u"),
        I32GtS => text("i32.gt_s"),
        I32GtU => text("i32.gt_u"),
        I32Add => text("i32.add"),
        I32Sub => text("i32.sub"),
        I32And => text("i32.and"),
        I32Or => text("i32.or"),
        I32Xor => text("i32.xor"),
        I32Shl => text("i32.shl"),
        I32ShrS => text("i32.shr_s"),
        I32ShrU => text("i32.shr_u"),
        I32WrapI64 => text("i32.wrap_i64"),
        I64Eqz => text("i64.eqz"),
        I64Eq => text("i64.eq"),
        I64LtS => text("i64.lt_s"),
        I64LtU => text("i64.lt_u"),
        I64GtS => text("i64.gt_s"),
        I64GtU => text("i64.gt_u"),
        I64Add => text("i64.add"),
        I64Sub => text("i64.sub"),
        I64And => text("i64.and"),
        I64Or => text("i64.or"),
        I64Xor => text("i64.xor"),
        I64Shl => text("i64.shl"),
        I64ShrS => text("i64.shr_s"),
        I64ShrU => text("i64.shr_u"),
        I64ExtendI32S => text("i64.extend_i32_s"),
        I64ExtendI32U => text("i64.extend_i32_u"),
        Br { label } => text(format!("br ${}", label)),
        BrIf { label } => text(format!("br.if ${}", label)),
        Return => text("return"),
        Block { label } => text(format!("block ${}", label)),
        Loop { label, block_type } => text(format!("loop ${}", label,)).append(match block_type {
            Some(block_type) => block_type_to_doc(block_type),
            None => nil(),
        }),
        If { .. } => unreachable!(),
        RefNull { ty } => text("ref.null ").append(heap_type_to_doc(ty)),
        RefFunc { name } => text(format!("ref.func ${}", name)),
        RefI31 => text("ref.i31"),
        I31GetS => text("i31.get_s"),
        I31GetU => text("i31.get_u"),
        RefAsNonNull => text("ref.as_non_null"),
        RefEq => text("ref.eq"),
        RefTest { ty } => text("ref.test ").append(ref_type_to_doc(ty)),
        RefCast { ty } => text("ref.cast ").append(ref_type_to_doc(ty)),
        Call { func } => text(format!("call ${}", func)),
        CallRef { type_idx } => text(format!("call_ref ${}", type_idx)),
        LocalGet { name } => text(format!("local.get ${}", name)),
        LocalSet { name } => text(format!("local.set ${}", name)),
        GlobalGet { name } => text(format!("global.get ${}", name)),
        GlobalSet { name } => text(format!("global.set ${}", name)),
        ArrayNewFixed { type_idx, len } => text(format!("array.new_fixed ${} {}", type_idx, len)),
        ArrayGet { ty } => text(format!("array.get ${}", ty)),
        ArrayGetU { ty } => text(format!("array.get_u ${}", ty)),
        ArraySet { ty } => text(format!("array.set ${}", ty)),
        ArrayLen => text("array.len"),
        StructNew { ty } => text(format!("struct.new ${}", ty)),
        StructGet { ty, field } => text(format!("struct.get ${} ${}", ty, field)),
        StructSet { ty, field } => text(format!("struct.set ${} ${}", ty, field)),
        Unreachable => text("unreachable"),
    }
}

// /// ```wat
// /// (block <label> <block_type>
// ///     <instr>*)
// /// ```
// fn block_to_doc(instr_seq_arena: &Arena<InstrSeq>, block: &Block) -> Doc {
//     let &Block { seq } = block;
//     let block_type = block_type_to_doc(&Unitype::UNITYPE.into_block_type_result());
//     let instr_seq = instr_seq_to_doc(instr_seq_arena, seq);
//     text("(block")
//         .append(
//             line().append(block_type).group(), // insert label here
//         )
//         .append(hardline())
//         .append(instr_seq)
//         .append(")")
//         .nest(INDENT)
//         .group()
// }

fn const_to_doc(ty: &NumType, val: i64) -> Doc {
    let instr = num_type_to_doc(ty).append(".const");
    let val = text(val.to_string());

    instr.append(space()).append(val)
}

// fn unop_to_doc(unop: &Unop) -> Doc {
//     let Unop { op } = unop;
//     let op = match op {
//         UnaryOp::I32Eqz => "i32.eqz",
//         UnaryOp::RefI31 => "ref.i31",
//         UnaryOp::I31GetS => "i31.get_s",
//         UnaryOp::I31GetU => "i31.get_u",
//         UnaryOp::ArrayLen => "array.len",
//         UnaryOp::RefAsNonNull => "ref.as_non_null",
//     };
//
//     text(format!("({})", op))
// }
//
// fn binop_to_doc(binop: &Binop) -> Doc {
//     let Binop { op } = binop;
//     let op = match op {
//         BinaryOp::I32Eq => "i32.eq",
//         BinaryOp::I32Add => "i32.add",
//     };
//
//     text(format!("({})", op))
// }

/// ```wat
/// (if <label> <block_type>
///     <instr>*
///     (then <instr>+)
///     (else <instr>+)?)
/// ```
fn if_instr_to_doc(if_instr: &Instr) -> Doc {
    let Instr {
        unfolded_instr,
        folded_instrs,
    } = if_instr;
    let UnfoldedInstr::If {
        label,
        block_type,
        then_block,
        else_block,
    } = unfolded_instr
    else {
        unreachable!()
    };

    let label = match label {
        None => nil(),
        Some(label) => text(format!("${}", label)),
    };

    let block_type = match block_type {
        None => nil(),
        Some(block_type) => line().append(block_type_to_doc(block_type)),
    };
    let predicate = instr_seq_to_doc(folded_instrs);
    let then_block = {
        let seq = instr_seq_to_doc(then_block);
        text("(then")
            .append(hardline())
            .append(seq)
            .append(")")
            .nest(INDENT)
    };
    let else_block = {
        let seq = instr_seq_to_doc(else_block);
        text("(else")
            .append(hardline())
            .append(seq)
            .append(")")
            .nest(INDENT)
    };

    text("(if")
        .append(label)
        .append(block_type)
        .append(hardline())
        .append(predicate)
        .append(hardline())
        .append(then_block)
        .append(hardline())
        .append(else_block)
        .append(")")
        .nest(INDENT)
}

// /// ```wat
// /// (ref.test <ty>)
// /// ```
// fn ref_test_to_doc(ref_test: &RefTest) -> Doc {
//     let RefTest { ty } = ref_test;
//     let ty = ref_type_to_doc(ty);
//     text("(ref.test")
//         .append(line())
//         .append(ty)
//         .append(")")
//         .nest(INDENT)
//         .group()
// }
//
// /// ```wat
// /// (ref.cast <ty>)
// /// ```
// fn ref_cast_to_doc(ref_cast: &RefCast) -> Doc {
//     let RefCast { result_ty } = ref_cast;
//     let result_ty = ref_type_to_doc(result_ty);
//     text("(ref.cast")
//         .append(line())
//         .append(result_ty)
//         .append(")")
//         .nest(INDENT)
//         .group()
// }

/// ```wat
/// (param <id> <val_type>)
/// ```
fn param_to_doc(param: &Param) -> Doc {
    let Param { name, ty } = param;

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
fn result_to_doc(ty: &ValType) -> Doc {
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
        Mutable::Mutable => text("(mut")
            .append(line())
            .append(val_type)
            .append(")")
            .nest(INDENT)
            .group(),
        Mutable::Const => val_type,
    }
}

fn block_type_to_doc(ty: &BlockType) -> Doc {
    match ty {
        BlockType::Result(ty) => result_to_doc(ty),
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
        // NumType::F32 => "f32",
        // NumType::F64 => "f64",
    })
}

/// ```wat
/// (ref null? <heap_type>)
/// ```
fn ref_type_to_doc(ty: &RefType) -> Doc {
    let RefType { null, heap_type } = ty;

    let nullability = match null {
        Nullable::NonNullable => nil(),
        Nullable::Nullable => space().append(text("null")),
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
        HeapType::Abs(ty) => abs_heap_type_to_doc(ty),
        HeapType::TypeIdx(ident) => text(format!("${}", ident)),
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
        Final::Final => line().append(text("final")),
        Final::NotFinal => nil(),
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

    let fields: Vec<Doc> = fields.iter().map(field_to_doc).collect();
    let fields = intersperse(fields, line());

    text("(struct")
        .append(line())
        .append(fields)
        .append(")")
        .nest(INDENT)
        .group()
}

fn field_to_doc(field: &Field) -> Doc {
    let Field { name, ty } = field;

    text("(field")
        .append(space())
        .append(text(format!("${}", name)))
        .append(line())
        .append(field_type_to_doc(ty))
        .append(")")
        .nest(INDENT)
        .group()
}

/// ```wat
/// (array <field_type>*)
/// ```
fn array_type_to_doc(ty: &ArrayType) -> Doc {
    let ArrayType { field_type } = ty;
    let field = field_type_to_doc(field_type);

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
    let params: Vec<Doc> = params.iter().map(|ty| param_to_doc(ty)).collect();
    let params = intersperse(params, line());

    let results: Vec<Doc> = results.iter().map(|ty| result_to_doc(ty)).collect();
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
    let FieldType { mutable, ty } = ty;
    let ty = storage_type_to_doc(ty);

    match mutable {
        Mutable::Const => ty,
        Mutable::Mutable => text("(mut")
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
