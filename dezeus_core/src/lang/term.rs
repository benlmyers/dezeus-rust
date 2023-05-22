use std::fmt::Display;

use snafu::prelude::*;

use super::language::Language;
use super::symbol::*;

pub struct Term {
    language: Language,
    sequence: Vec<Symbol>,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display(
        "Unable to find symbol {} at position {} in the language.",
        symbol,
        position
    ))]
    SymbolNotInLanguage { symbol: Symbol, position: i8 },
}

impl Term {
    pub fn validate_each_symbol(&self) -> i8 {
        let mut position = 0;
        for symbol in self.sequence.iter() {
            if !self.language.symbols().contains(symbol) {
                return position;
            }
            position += 1;
        }
        return -1;
    }
}

impl Term {
    pub fn new(language: Language, sequence: Vec<Symbol>) -> Result<Self, Error> {
        let term = Term { language, sequence };
        if term.validate_each_symbol() >= 0 {
            return Err(Error::SymbolNotInLanguage {
                symbol: term.sequence[term.validate_each_symbol() as usize].clone(),
                position: term.validate_each_symbol(),
            });
        }
        Ok(term)
    }
    pub fn sequence(&self) -> Vec<Symbol> {
        self.sequence.clone()
    }
    pub fn language(&self) -> &Language {
        &self.language
    }
}

use super::Formalize;

impl Formalize for Term {
    fn formalize(&self) -> String {
        let mut string = String::new();
        for symbol in self.sequence.iter() {
            string.push_str(&symbol.formalize());
            string.push(' ');
        }
        string.pop();
        string
    }
}

impl Display for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Formalize::formalize(self))
    }
}
