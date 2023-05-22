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
    pub fn new(string: String, variant: Variant) -> Self {
        Self {
            string,
            variant,
            arity: 0,
        }
    }
    pub fn new_with_arity(string: String, variant: Variant, arity: i8) -> Self {
        Self {
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
