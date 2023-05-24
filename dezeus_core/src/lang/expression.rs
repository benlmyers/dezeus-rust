use std::fmt::Result as FmtResult;
use std::fmt::{Debug, Display};

use super::language::Language;
use super::symbol::*;

#[derive(Clone, PartialEq, Eq)]
pub struct Expression {
    language: Language,
    sequence: Vec<Symbol>,
}

#[derive(PartialEq)]
pub enum Error {
    Empty,
    SymbolNotInLanguage {
        expression: Expression,
        symbol: Symbol,
        position: i8,
    },
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Empty => write!(f, "Empty expression."),
            Error::SymbolNotInLanguage {
                symbol,
                position,
                expression,
            } => {
                write!(
                    f,
                    "The symbol \'{}\' at position {} in \"{}\" is not present in the expression's language.",
                    symbol, position, expression
                )
            }
        }
    }
}

impl Expression {
    pub fn new(language: Language, sequence: Vec<Symbol>) -> Result<Self, Error> {
        let expression = Expression { language, sequence };
        if expression.find_invalid_symbol() >= 0 {
            return Err(Error::SymbolNotInLanguage {
                symbol: expression.sequence[expression.find_invalid_symbol() as usize].clone(),
                position: expression.find_invalid_symbol(),
                expression: expression.clone(),
            });
        }
        if expression.size() == 0 {
            return Err(Error::Empty);
        }
        Ok(expression)
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

    fn find_invalid_symbol(&self) -> i8 {
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

use super::Formalize;

impl Formalize for Expression {
    fn formalize(&self) -> String {
        let mut result = String::new();
        for symbol in self.sequence.iter() {
            result.push_str(&symbol.formalize());
        }
        result
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", Formalize::formalize(self))
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", Formalize::formalize(self))
    }
}
