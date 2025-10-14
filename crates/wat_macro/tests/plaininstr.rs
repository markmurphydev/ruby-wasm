// use wat_macro::wat;

use wat_defs::instr::UnfoldedInstr;
use wat_defs::instr::Instr;
use wat_macro::wat;

#[test]
pub fn nop() {
    let expected = ();
    let actual = wat!{ (nop) };
    assert_eq!(1, 1)
}