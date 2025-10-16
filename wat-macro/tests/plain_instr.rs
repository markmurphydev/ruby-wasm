use wat_defs::instr::Instr;
use expect_test::expect;
use wat_macro::wat;

#[test]
pub fn nop() {
    let actual = wat! { (nop) };
    let actual = &format!("{:?}", actual);
    let expected = expect!["Instr { instr: Nop, folded_instrs: [] }"];
    expected.assert_eq(actual);
}

#[test]
pub fn nop_with_folded_instrs() {
    let actual = wat! { (nop (nop) (nop (nop)) (nop)) };
    let actual = &format!("{:?}", actual);
    let expected = expect!["Instr { instr: Nop, folded_instrs: [Instr { instr: Nop, folded_instrs: [] }, Instr { instr: Nop, folded_instrs: [Instr { instr: Nop, folded_instrs: [] }] }, Instr { instr: Nop, folded_instrs: [] }] }"];
    expected.assert_eq(actual);
}

#[test]
pub fn nop_quasi_quoted() {
    let quoted: Instr = wat_defs::instr::Instr {
        instr: wat_defs::instr::UnfoldedInstr::Nop,
        folded_instrs: vec![],
    };
    let actual: Instr = wat! { ,(quoted) };
    let actual = &format!("{:?}", actual);
    let expected = expect!["Instr { instr: Nop, folded_instrs: [] }"];
    expected.assert_eq(actual);
}

#[test]
pub fn nop_quasi_quote_wat() {
    let actual: Instr = wat! { ,(wat! { (nop) }) };
    let actual = &format!("{:?}", actual);
    let expected = expect!["Instr { instr: Nop, folded_instrs: [] }"];
    expected.assert_eq(actual);
}
