use dezeus_core::lang::{symbol::*, Formalize};

#[test]
fn declare_symbol_test() {
    let symbol = Symbol::new(String::from("a"), Variant::Constant);
    assert_eq!(symbol.string(), String::from("a"));
    assert_eq!(symbol.formalize(), String::from("a"));
    assert_eq!(symbol.variant(), Variant::Constant);
    assert_eq!(symbol.arity(), 0);
    let symbol = Symbol::new_with_arity(String::from("f"), Variant::Function, 2);
    assert_eq!(symbol.string(), String::from("f"));
    assert_eq!(symbol.formalize(), String::from("f"));
    assert_eq!(symbol.variant(), Variant::Function);
    assert_eq!(symbol.arity(), 2);
}
