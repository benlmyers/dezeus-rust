use super::symbol::*;

pub struct Language {
    symbols: Vec<Symbol>,
}

impl Language {
    pub fn symbols(&self) -> Vec<Symbol> {
        self.symbols.clone()
    }
    pub fn size(&self) -> usize {
        self.symbols.len()
    }
}

impl Language {
    pub fn l() -> Self {
        Language {
            symbols: vec![
                Symbol::new(0, String::from("("), Variant::Grouping),
                Symbol::new(1, String::from(")"), Variant::Grouping),
                Symbol::new(2, String::from(","), Variant::Grouping),
                Symbol::new(3, String::from("¬"), Variant::Logical),
                Symbol::new(4, String::from("→"), Variant::Logical),
            ],
        }
    }
}
