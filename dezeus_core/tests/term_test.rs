mod common;

use dezeus_core::lang;
use dezeus_core::lang::expression::Expression;
use dezeus_core::lang::language::*;
use dezeus_core::lang::symbol::*;
use dezeus_core::lang::term::Term;

#[test]
fn define_atomic_test() {
    let a = Symbol::constant(String::from("a"));
    let b = Symbol::constant(String::from("b"));
    let x = Symbol::variable(String::from("x"));
    let _ = Symbol::variable(String::from("y"));
    let f = Symbol::function(String::from("f"), 2);
    let l1 = lang!(a, x, f);
    let e1 = Expression::new(l1.clone(), vec![a.clone()]).unwrap();
    let _ = Term::new(e1.clone()).unwrap();
    assert!(Expression::new(l1.clone(), vec![b.clone()]).is_err());
    let e2 = Expression::new(l1.clone(), vec![x.clone()]).unwrap();
    let _ = Term::new(e2).unwrap();
    let e3 = Expression::new(
        l1.clone(),
        vec![
            f.clone(),
            Symbol::left_paren(),
            a.clone(),
            Symbol::comma(),
            x.clone(),
            Symbol::right_paren(),
        ],
    )
    .unwrap();
    let _ = Term::new(e3).unwrap();
}
