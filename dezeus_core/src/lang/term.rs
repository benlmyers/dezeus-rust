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
    pub fn new(language: Language, sequence: Vec<Symbol>) -> Result<Self, Error> {
        let term = Term { language, sequence };
        if term.val_each_symbol() >= 0 {
            return Err(Error::SymbolNotInLanguage {
                symbol: term.sequence[term.val_each_symbol() as usize].clone(),
                position: term.val_each_symbol(),
            });
        }
        if term.size() == 1 {
            if term.is_variable() {
                return Ok(term);
            }
            if term.is_constant() {
                return Ok(term);
            }
            return Err(Error::SymbolNotInLanguage {
                symbol: term.sequence[0].clone(),
                position: 0,
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

    pub fn size(&self) -> usize {
        self.sequence.len()
    }

    fn val_each_symbol(&self) -> i8 {
        let mut position = 0;
        for symbol in self.sequence.iter() {
            if !self.language.symbols().contains(symbol) {
                return position;
            }
            position += 1;
        }
        return -1;
    }

    fn is_variable(&self) -> bool {
        if self.size() != 1 {
            return false;
        }
        self.sequence[0].variant() == Variant::Variable
    }

    fn is_constant(&self) -> bool {
        if self.size() != 1 {
            return false;
        }
        self.sequence[0].variant() == Variant::Constant
    }

    fn is_composite(&self) -> bool {
        if self.size() < 4 {
            return false;
        }
        if self.sequence[1] != Symbol::left_paren() {
            return false;
        }
        if self.sequence[self.size() - 1] != Symbol::right_paren() {
            return false;
        }
        if self.sequence[0].variant() != Variant::Function {
            return false;
        }
        let mut term_vec: Vec<&Symbol> = Vec::new();
        let mut param_count = 0;
        for symbol in self.sequence.iter().skip(1) {
            if symbol == &Symbol::comma() {
                match Term::new(
                    self.language.clone(),
                    term_vec.iter().cloned().cloned().collect(),
                ) {
                    Ok(_) => {
                        param_count += 1;
                        term_vec.clear();
                    }
                    Err(_) => {
                        return false;
                    }
                }
            } else {
                term_vec.push(symbol);
            }
        }
        param_count == self.sequence[0].arity()
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
