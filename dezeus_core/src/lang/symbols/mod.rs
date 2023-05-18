pub mod connective;
pub mod constant;
pub mod function;
pub mod relation;
pub mod variable;

pub trait Symbol {
    fn formal(&self) -> String;
}
