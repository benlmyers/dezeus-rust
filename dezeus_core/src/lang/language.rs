use std::collections::HashSet;

use super::symbol::*;

pub struct Language {
    symbols: HashSet<Symbol>,
}

impl Language {
    fn new(symbols: HashSet<Symbol>) -> Self {
        return Language::l().extend(symbols);
    }
    fn extend(&self, symbols: HashSet<Symbol>) -> Self {
        let mut new_symbols = self.symbols.clone();
        new_symbols.extend(symbols);
        return Language {
            symbols: new_symbols,
        };
    }
}

#[macro_export]
macro_rules! lang {
    ( $( $x:expr ),* ) => {
        {
            let mut symbols = HashSet::new();
            $(
                symbols.insert($x);
            )*
            Language::new(symbols)
        }
    };
}

impl Language {
    pub fn symbols(&self) -> HashSet<Symbol> {
        self.symbols.clone()
    }
    pub fn size(&self) -> usize {
        self.symbols.len()
    }
    pub fn l() -> Self {
        lang!(Symbol::left_paren(), Symbol::right_paren())
    }
}
