pub mod expression;
pub mod language;
pub mod symbol;
pub mod term;

pub trait Formalize {
    fn formalize(&self) -> String;
}
