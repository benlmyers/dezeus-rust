#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Symbol {
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
    pub fn left_paren() -> Self {
        Self::new(String::from("("), Variant::Grouping)
    }

    pub fn right_paren() -> Self {
        Self::new(String::from(")"), Variant::Grouping)
    }

    pub fn comma() -> Self {
        Self::new(String::from(","), Variant::Grouping)
    }

    pub fn not() -> Self {
        Self::new(String::from("¬"), Variant::Logical)
    }

    pub fn implies() -> Self {
        Self::new(String::from("→"), Variant::Logical)
    }

    pub fn for_all() -> Self {
        Self::new(String::from("∀"), Variant::Logical)
    }

    pub fn equals() -> Self {
        Self::new(String::from("="), Variant::Logical)
    }
}

impl Symbol {
    pub fn string(&self) -> String {
        self.string.clone()
    }

    pub fn variant(&self) -> Variant {
        self.variant.clone()
    }

    pub fn arity(&self) -> i8 {
        self.arity
    }

    pub fn constant(string: String) -> Self {
        Self::new(string, Variant::Constant)
    }

    pub fn variable(string: String) -> Self {
        Self::new(string, Variant::Variable)
    }

    pub fn function(string: String, arity: i8) -> Self {
        Self::new_with_arity(string, Variant::Function, arity)
    }

    fn new(string: String, variant: Variant) -> Self {
        Self {
            string,
            variant,
            arity: 0,
        }
    }

    fn new_with_arity(string: String, variant: Variant, arity: i8) -> Self {
        Self {
            string,
            variant,
            arity,
        }
    }
}

use super::Formalize;

impl Formalize for Symbol {
    fn formalize(&self) -> String {
        self.string.clone()
    }
}

use std::fmt::Display;

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Formalize::formalize(self))
    }
}
