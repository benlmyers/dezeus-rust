use super::symbol::*;

pub struct Language {
    symbols: Vec<Symbol>,
}

impl Language {
    pub fn l() -> Self {
        Language {
            symbols: vec![
                Symbol::left_paren(),
                Symbol::right_paren(),
                Symbol::comma(),
                Symbol::not(),
                Symbol::implies(),
                Symbol::for_all(),
                Symbol::equals(),
            ],
        }
    }
}

impl Language {
    pub fn extend(&self, symbols: Vec<Symbol>) -> Self {
        let mut new_symbols = self.symbols.clone();
        for symbol in symbols.iter() {
            new_symbols.push(symbol.clone());
        }
        Language {
            symbols: new_symbols,
        }
    }
    pub fn new(symbols: Vec<Symbol>) -> Self {
        return Language::l().extend(symbols);
    }
    pub fn symbols(&self) -> Vec<Symbol> {
        self.symbols.clone()
    }
    pub fn size(&self) -> usize {
        self.symbols.len()
    }
}
