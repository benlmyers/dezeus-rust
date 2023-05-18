use std::collections::HashSet;

use super::symbol::Symbol;

pub struct Language {
    symbols: HashSet<Symbol>,
}

impl Language {
    pub fn new(symbols: Vec<Symbol>) -> Self {
        let mut set = HashSet::new();
        for symbol in symbols {
            set.insert(symbol);
        }
        Self { symbols: set }
    }
    pub fn symbols(&self) -> HashSet<Symbol> {
        self.symbols.clone()
    }
    pub fn size(&self) -> i8 {
        self.symbols.len() as i8
    }
}
