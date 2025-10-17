use expect_test::expect;
use wat_defs::instr::Instr;
use wat_macro::wat;

#[test]
pub fn nop() {
    let actual = wat! { (nop) };
    let actual = &format!("{:?}", actual);
    let expected = expect!["[Instr { unfolded_instr: Nop, folded_instrs: [] }]"];
    expected.assert_eq(actual);
}

#[test]
pub fn nop_with_folded_instrs() {
    let actual = wat! { (nop (nop) (nop (nop)) (nop)) };
    let actual = &format!("{:?}", actual);
    let expected = expect!["[Instr { unfolded_instr: Nop, folded_instrs: [Instr { unfolded_instr: Nop, folded_instrs: [] }, Instr { unfolded_instr: Nop, folded_instrs: [Instr { unfolded_instr: Nop, folded_instrs: [] }] }, Instr { unfolded_instr: Nop, folded_instrs: [] }] }]"];
    expected.assert_eq(actual);
}

#[test]
pub fn nop_quasi_quoted() {
    let quoted: Vec<Instr> = vec![wat_defs::instr::Instr {
        unfolded_instr: wat_defs::instr::UnfoldedInstr::Nop,
        folded_instrs: vec![],
    }];
    let actual: Vec<Instr> = wat![ ,(quoted) ];
    let actual = &format!("{:?}", actual);
    let expected = expect!["[Instr { unfolded_instr: Nop, folded_instrs: [] }]"];
    expected.assert_eq(actual);
}

#[test]
pub fn nop_quasi_quote_wat() {
    let actual: Vec<Instr> = wat![ ,(wat! { (nop) }) ];
    let actual = &format!("{:?}", actual);
    let expected = expect!["[Instr { unfolded_instr: Nop, folded_instrs: [] }]"];
    expected.assert_eq(actual);
}


#[test]
pub fn array_new_fixed() {
    let actual: Vec<Instr> = wat![ (array_new_fixed $arr_unitype 0) ];
    let actual = &format!("{:?}", actual);
    let expected = expect![[r#"[Instr { unfolded_instr: ArrayNewFixed { type_idx: "arr_unitype", len: 0 }, folded_instrs: [] }]"#]];
    expected.assert_eq(actual);
}
