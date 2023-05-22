#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
    id: i8,
    string: String,
    variant: Variant,
    arity: i8,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Variant {
    Logical,
    Grouping,
    Variable,
    Constant,
    Function,
    Predicate,
}

impl Symbol {
    pub const fn new(id: i8, string: String, variant: Variant) -> Self {
        Self {
            id,
            string,
            variant,
            arity: 0,
        }
    }
    pub fn new_with_arity(id: i8, string: String, variant: Variant, arity: i8) -> Self {
        Self {
            id,
            string,
            variant,
            arity,
        }
    }
    pub fn string(&self) -> String {
        self.string.clone()
    }
    pub fn variant(&self) -> Variant {
        self.variant.clone()
    }
    pub fn arity(&self) -> i8 {
        self.arity
    }
}

use super::Formalize;

impl Formalize for Symbol {
    fn formalize(&self) -> String {
        self.string.clone()
    }
}
