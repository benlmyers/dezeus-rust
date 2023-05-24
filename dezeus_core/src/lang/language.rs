use std::collections::HashSet;

use super::symbol::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language {
    symbols: HashSet<Symbol>,
}

impl Language {
    fn new(symbols: HashSet<Symbol>) -> Self {
        return Language { symbols };
    }
}

#[macro_export]
macro_rules! lang {
    ( $( $x:expr ),* ) => {
        {
            use std::collections::HashSet;
            let mut symbols = HashSet::new();
            $(
                symbols.insert($x.clone());
            )*
            Language::l().extend(symbols)
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
        let mut set = HashSet::new();
        set.insert(Symbol::left_paren());
        set.insert(Symbol::right_paren());
        set.insert(Symbol::comma());
        set.insert(Symbol::not());
        set.insert(Symbol::implies());
        set.insert(Symbol::for_all());
        set.insert(Symbol::equals());
        Language::new(set)
    }

    pub fn extend(&self, symbols: HashSet<Symbol>) -> Self {
        let mut new_symbols = self.symbols.clone();
        new_symbols.extend(symbols);
        return Language {
            symbols: new_symbols,
        };
    }
}
