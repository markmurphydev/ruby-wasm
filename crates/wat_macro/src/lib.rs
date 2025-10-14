use quote::{quote, ToTokens};
use syn::parse::Parser;
use syn::parse_macro_input;
use wat_defs::instr::Instr;

// #[macro_export]
// macro_rules! wat {
//     { ( nop $( ( , $folded_instr:tt ) )* $(,)?) } => {
//         $crate::Instr {
//             instr: $crate::UnfoldedInstr::Nop,
//             folded_instrs: vec![$(wat! { ( $folded_instr ) }),*]
//         }
//     };
//     { ( const.i32, $val:expr $(, ( $folded_instr:tt ) )* $(,)?) } => {
//         $crate::Instr {
//             instr: $crate::UnfoldedInstr::Const {
//                 ty: $crate::NumType::I32,
//                 val: $val
//             },
//             folded_instrs: vec![$(wat! { ( $folded_instr ) }),*]
//         }
//     };
//     { (if
//         $(, #$label:ident)?
//         $(, (type #$ty:ident))?
//         , (then $(, (then_instr:tt))* $(,)?)
//         $(, (else $(, (else_instr:tt))* $(,)?))?
//     ) } => {
//         44
//     };
//     { (if
//         $(, #$label:ident)?
//         $(, (result $ty:tt))?
//         , (then $(, (then_instr:tt))* $(,)?)
//         $(, (else $(, (else_instr:tt))* $(,)?))?
//     ) } => {
//         44
//     };
//     // What about bare sequences of instructions? Maybe this:
//     // [ $(($tt:tt))* ] => [ $(wat!($tt)),* ]
// }

#[proc_macro]
pub fn wat(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let res: Instr = parse_macro_input!(input as Instr);
    res.to_token_stream().into()
    // proc_macro::TokenStream::new()
}
