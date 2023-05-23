mod common;

use dezeus_core::lang;
use dezeus_core::lang::language::*;
use dezeus_core::lang::symbol::*;
use dezeus_core::lang::term::Term;

#[test]
fn define_atomic_test() {
    let a = Symbol::constant(String::from("a"));
    let b = Symbol::constant(String::from("b"));
    let x = Symbol::variable(String::from("x"));
    let y = Symbol::variable(String::from("y"));
    let f = Symbol::function(String::from("f"), 2);
    let l1 = lang!(a.clone(), x.clone(), f.clone());
    let t1 = Term::new(l1.clone(), vec![a.clone()]);
    assert!(t1.is_ok());
    let t2 = Term::new(l1.clone(), vec![b.clone()]);
    assert!(t2.is_err());
    let t3 = Term::new(l1.clone(), vec![x.clone()]);
    assert!(t3.is_ok());
    let t4 = Term::new(
        l1.clone(),
        vec![
            f.clone(),
            Symbol::left_paren(),
            a.clone(),
            Symbol::comma(),
            x.clone(),
            Symbol::right_paren(),
        ],
    );
    assert!(t4.is_ok());
}
