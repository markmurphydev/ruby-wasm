use std::hash::{DefaultHasher, Hash, Hasher};
use wat_defs::instr::Instr;
use crate::node::{And, Array, Call, ConstantRead, Expr, GlobalVariableRead, GlobalVariableWrite, If, Or, Program, Statements, Subsequent, Until, While};
use crate::unitype::Unitype;
use wat_defs::module::Module;
use wat_macro::wat;
use crate::corelib;
use crate::corelib::class::Class;

pub const RUBY_TOP_LEVEL_FUNCTION_NAME: &str = "__ruby_top_level_function";

pub struct CompileCtx {
    pub module: Module,
}

impl CompileCtx {
    pub fn new(module: Module) -> CompileCtx {
        CompileCtx { module }
    }
}

pub fn compile(ctx: &mut CompileCtx, program: &Program) {
    // TODO: exported.
    let stmts = compile_program(ctx, program);
    let top_level_func = wat! {
        (func ,(RUBY_TOP_LEVEL_FUNCTION_NAME.to_string())
            (export ,(RUBY_TOP_LEVEL_FUNCTION_NAME.to_string()))
            (result (ref eq))
            ,(stmts))
    };
    ctx.module.funcs.push(top_level_func);
}

fn compile_program(ctx: &mut CompileCtx, program: &Program) -> Vec<Instr> {
    compile_statements(ctx, &program.statements)
}

fn compile_statements(ctx: &mut CompileCtx, statements: &Statements) -> Vec<Instr> {
    let Statements { body } = statements;

    // In Ruby, every expression returns a value or nil.
    // If there are no statements, return nil.
    // Suppress all values except the last.
    if body.is_empty() {
        i31_const(Unitype::NIL_BIT_PATTERN)
    } else {
        let mut stmts = vec![];
        for expr in body.iter() {
            stmts.append(&mut compile_expr(ctx, expr));
        }
        stmts
    }
}

fn compile_expr(ctx: &mut CompileCtx, expr: &Expr) -> Vec<Instr> {
    match expr {
        &Expr::Integer(n) => compile_integer(ctx, n),
        Expr::SingleQuoteString(s) => compile_single_quote_string(ctx, s),
        Expr::False => i31_const(Unitype::FALSE_BIT_PATTERN),
        Expr::True => i31_const(Unitype::TRUE_BIT_PATTERN),
        Expr::Nil => i31_const(Unitype::NIL_BIT_PATTERN),
        Expr::GlobalVariableWrite(global_write) => compile_global_variable_write(ctx, global_write),
        Expr::GlobalVariableRead(global_read) => {
            compile_global_variable_read(global_read)
        }
        Expr::ConstantWrite(_constant_write) => todo!(),
        Expr::ConstantRead(constant_read_expr) => {
            compile_constant_read_expr(ctx, &*constant_read_expr)
        }

        Expr::If(if_expr) => compile_if_expr(ctx, &*if_expr),
        Expr::While(while_expr) => compile_while_expr(ctx, &*while_expr),
        Expr::Until(until_expr) => compile_until_expr(ctx, &*until_expr),
        Expr::Call(call_expr) => compile_call_expr(ctx, &*call_expr),
        Expr::And(and_expr) => compile_and_expr(ctx, &*and_expr),
        Expr::Or(or_expr) => compile_or_expr(ctx, &*or_expr),
        Expr::Array(arr_expr) => compile_arr_expr(ctx, &*arr_expr),
    }
}

fn compile_arr_expr(ctx: &mut CompileCtx, arr_expr: &Array) -> Vec<Instr> {
    let Array { vals } = arr_expr;
    let vals: Vec<Instr> = vals.into_iter().map(|val| compile_expr(ctx, val)).flatten().collect();
    wat!{
        (array_new_fixed $arr_unitype ,(vals.len() as i64)
            ,(vals)
        )
    }
}

fn compile_and_expr(ctx: &mut CompileCtx, and_expr: &And) -> Vec<Instr> {
    let And { lhs, rhs } = and_expr;
    let wat_args = {
        let mut res = compile_expr(ctx, lhs);
        res.append(&mut compile_expr(ctx, rhs));
        res
    };
    wat![ (call $and ,(wat_args)) ]
}

fn compile_or_expr(ctx: &mut CompileCtx, or_expr: &Or) -> Vec<Instr> {
    let Or { lhs, rhs } = or_expr;
    let wat_args = {
        let mut res = compile_expr(ctx, lhs);
        res.append(&mut compile_expr(ctx, rhs));
        res
    };
    wat![ (call $or ,(wat_args)) ]
}

/// Convert the given integer into a Wasm fixnum or const global representation
fn compile_integer(ctx: &mut CompileCtx, n: i64) -> Vec<Instr> {
    let unitype = Unitype::from_integer(n);
    match unitype {
        fixnum @ Unitype::Fixnum(_) => {
            i31_const(fixnum.to_i31_bits())
        }
        Unitype::HeapNum(heapnum) => {
            // `heapnum` is a constant value.
            // So, create a global and get its value.

            // TODO -- We need to dedup constant heapnums.
            //  If you have 2 of the same-valued global this probably breaks at validation time.
            let global_id = format!("global_i32_{}", heapnum);
            let global = wat![ (global ,(global_id) (ref eq) (const_i64 ,(heapnum))) ];
            ctx.module.globals.push(global);

            wat![ (global_get ,(global_id)) ]
        }
        _ => unreachable!(),
    }
}

fn compile_single_quote_string(ctx: &mut CompileCtx, str: &String) -> Vec<Instr> {
    // TODO -- Dedup strings.
    let mut hasher = DefaultHasher::new();
    str.hash(&mut hasher);
    let global_id = format!("single_quote_string_{}", hasher.finish());

    let bytes: Vec<_> = str.bytes().map(|b| wat![ (const_i32 ,(b as i64)) ]).flatten().collect();
    let len = bytes.len() as i64;
    let global = wat! {
        (global ,(global_id.clone()) (ref $str)
            (array_new_fixed $str ,(len)
                ,(bytes)))
    };
    ctx.module.globals.push(global);

    wat![ (global_get ,(global_id)) ]
}

/// Add a global to the Module, setting its value to the write's rhs.
fn compile_global_variable_write(
    ctx: &mut CompileCtx,
    global_write: &GlobalVariableWrite,
) -> Vec<Instr> {
    let GlobalVariableWrite { name, expr } = global_write;

    if !ctx.module.globals.iter().any(|glob| glob.name == *name) {
        let global = wat![ (global ,(name) (mut (ref eq)) )];
        ctx.module.globals.push(global);
    }

    let rhs = compile_expr(ctx, expr);
    wat![ (global_set ,(name.to_string()) ,(rhs)) ]
}

fn compile_global_variable_read(
    global_read: &GlobalVariableRead,
) -> Vec<Instr> {
    let GlobalVariableRead { name } = global_read;
    wat![ (global_get ,(name.to_string())) ]
}

fn compile_constant_read_expr(
    ctx: &mut CompileCtx,
    constant_read_expr: &ConstantRead,
) -> Vec<Instr> {
    // TODO -- Assuming all constants are classes.
    let ConstantRead { name } = constant_read_expr;
    let name = Class::name_to_identifier(name);
    wat![ (global_get ,(name)) ]
}

fn compile_if_expr(
    ctx: &mut CompileCtx,
    if_expr: &If,
) -> Vec<Instr> {
    let If {
        predicate,
        statements,
        subsequent,
    } = if_expr;
    let else_branch = match subsequent {
        Subsequent::None => i31_const(Unitype::NIL_BIT_PATTERN),
        Subsequent::Elsif(if_expr) => compile_if_expr(ctx, &if_expr),
        Subsequent::Else(else_expr) => compile_statements(ctx, &else_expr.statements),
    };

    wat! {
        (if (result (ref eq))
            ,(compile_expr_to_wasm_predicate(ctx, predicate))
            (then ,(compile_statements(ctx, statements)))
            (else ,(else_branch))
        )
    }
}

fn compile_while_expr(
    ctx: &mut CompileCtx,
    while_expr: &While,
) -> Vec<Instr> {
    let While {
        predicate,
        statements,
    } = while_expr;
    let predicate = compile_expr_to_wasm_predicate(ctx, predicate);
    let stmts = compile_statements(ctx, statements);

    wat! {
        (loop $while
            (if (result (ref eq))
                ,(predicate)
                (then ,(stmts))
                (else (br $while))
            )
        )
    }
}

fn compile_until_expr(
    ctx: &mut CompileCtx,
    until_expr: &Until,
) -> Vec<Instr> {
    // TODO -- It might be nicer to have an IR where `until` is lowered to `while`
    let Until {
        predicate,
        statements,
    } = until_expr;
    let predicate = compile_expr_to_wasm_predicate(ctx, predicate);
    let stmts = compile_statements(ctx, statements);
    wat! {
        (loop $until
            (i32_eqz ,(predicate))
            (then ,(stmts))
            (br $label)
        )
    }
}

fn compile_call_expr(ctx: &mut CompileCtx, call_expr: &Call) -> Vec<Instr> {
    let Call { receiver, name, args } = call_expr;

    match name.as_str() {
        "+" => {
            assert_eq!(1, args.len());
            compile_binop(ctx, wat!($add), receiver, &args[0])
        },
        "-@" => {
            assert!(args.is_empty());
            wat![ (call $negate ,(compile_expr(ctx, receiver)))]
        }
        ">" => {
            assert_eq!(1, args.len());
            compile_binop(ctx, wat!($gt), receiver, &args[0])
        }
        "<" => {
            assert_eq!(1, args.len());
            compile_binop(ctx, wat!($lt), receiver, &args[0])
        }
        _ => {
            let name = corelib::global::string_identifier(name);
            let mut receiver = compile_expr(ctx, receiver);

            let mut message = wat!{
                (global_get ,(name))
            };

            let args: Vec<_> = args.iter().map(|arg| compile_expr(ctx, arg)).flatten().collect();
            let mut args = wat! {
                (array_new_fixed $arr_unitype ,(args.len() as i64)
                    ,(args))
            };
            let wat_args = {
                receiver.append(&mut message);
                receiver.append(&mut args);
                receiver
            };
            wat! {
                (call $call
                    ,(wat_args))
            }
        }
    }
}

fn compile_binop(ctx: &mut CompileCtx, name: String, lhs: &Expr, rhs: &Expr) -> Vec<Instr> {
    let mut lhs = compile_expr(ctx, lhs);
    let mut rhs = compile_expr(ctx, rhs);
    let wat_args ={
        lhs.append(&mut rhs);
        lhs
    };

    wat! {
        (call ,(name)
            ,(wat_args))
    }
}

/// Turns a Ruby Expr into a Wasm predicate.
/// A ruby Expr evaluates to a ruby-value (True, False, Nil, ...)
/// To use as a Wasm predicate, we need to test whether the result is truthy or not.
/// TODO -- right now we pretend that "truthy" is "not-false"
fn compile_expr_to_wasm_predicate(ctx: &mut CompileCtx, expr: &Expr) -> Vec<Instr> {
    let expr = compile_expr(ctx, expr);
    wat! {
        (i32_eqz (call $is_false ,(expr)))
    }
}

fn i31_const(bits: i32) -> Vec<Instr> {
    wat![ (ref_i31 (const_i32 ,(bits.into()))) ]
}
