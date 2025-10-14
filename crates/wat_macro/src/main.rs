use wat_defs::instr::Instr;
use wat_defs::instr::UnfoldedInstr;
use wat_macro::wat;

fn add_1(x: i64) -> i64 {
    x + 1
}

fn main() {
    let x = wat!{ (nop) };

    // eprintln!("{:?}", w);
}