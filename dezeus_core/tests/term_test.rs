mod common;

use dezeus_core::expr;
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
    let y = Symbol::variable(String::from("y"));
    let f = Symbol::function(String::from("f"), 2);
    let g = Symbol::function(String::from("g"), 1);
    let l1 = lang!(a, x, f);
    let l2 = lang!(a, b, x, y, f, g);
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
    let e4 = expr!(l2, g, Symbol::left_paren(), y, Symbol::right_paren());
    let _ = Term::new(e4).unwrap();
}
