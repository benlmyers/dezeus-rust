use dezeus_core::lang::symbol::*;
use dezeus_core::lang::term::*;

#[test]
fn declare_term_test() {
    let term = Term::new(vec![
        Symbol::new(String::from("f"), Variant::Function),
        Symbol::new(String::from("a"), Variant::Constant),
        Symbol::new(String::from("b"), Variant::Constant),
    ]);
}
