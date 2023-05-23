use std::fmt::Result as FmtResult;
use std::fmt::{Debug, Display};

use super::language::Language;
use super::symbol::*;

pub struct Term {
    language: Language,
    sequence: Vec<Symbol>,
}

pub enum Error {
    SymbolNotInLanguage {
        symbol: Symbol,
        position: i8,
    },
    InvalidConstruction {
        position: i8,
    },
    InvalidSubterm {
        item: Vec<Symbol>,
        cause: Box<Error>,
    },
    InvalidArity {
        expected: i8,
        found: i8,
    },
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::SymbolNotInLanguage { symbol, position } => {
                write!(
                    f,
                    "Symbol {} not in language at position {}",
                    symbol, position
                )
            }
            Error::InvalidConstruction { position } => {
                write!(f, "Invalid construction at position {}", position)
            }
            Error::InvalidSubterm { item, cause } => {
                write!(f, "Invalid subterm {:?}: {:?}", item, cause)
            }
            Error::InvalidArity { expected, found } => {
                write!(f, "Invalid arity: expected {}, found {}", expected, found)
            }
        }
    }
}

impl Term {
    pub fn new(language: Language, sequence: Vec<Symbol>) -> Result<Self, Error> {
        let term = Term { language, sequence };
        if term.find_invalid_symbol() >= 0 {
            return Err(Error::SymbolNotInLanguage {
                symbol: term.sequence[term.find_invalid_symbol() as usize].clone(),
                position: term.find_invalid_symbol(),
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
        match term.val_composite() {
            Ok(_) => Ok(term),
            Err(cause) => Err(cause),
        }
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

    fn val_composite(&self) -> Result<(), Error> {
        if self.size() < 4 {
            return Err(Error::InvalidConstruction {
                position: (self.size() - 1) as i8,
            });
        }
        if self.sequence[1] != Symbol::left_paren() {
            return Err(Error::InvalidConstruction { position: 1 });
        }
        if self.sequence[self.size() - 1] != Symbol::right_paren() {
            return Err(Error::InvalidConstruction {
                position: (self.size() - 1) as i8,
            });
        }
        if self.sequence[0].variant() != Variant::Function {
            return Err(Error::InvalidConstruction { position: 0 });
        }
        let mut term_vec: Vec<&Symbol> = Vec::new();
        let mut param_count = 0;
        for symbol in self.sequence.iter().skip(2) {
            if symbol == &Symbol::comma() || symbol == &Symbol::right_paren() {
                match Term::new(
                    self.language.clone(),
                    term_vec.iter().cloned().cloned().collect(),
                ) {
                    Ok(_) => {
                        param_count += 1;
                        term_vec.clear();
                    }
                    Err(e) => {
                        return Err(Error::InvalidSubterm {
                            item: term_vec.iter().cloned().cloned().collect(),
                            cause: Box::new(e),
                        })
                    }
                }
            } else {
                term_vec.push(symbol);
            }
        }
        if param_count != self.sequence[0].arity() {
            return Err(Error::InvalidArity {
                expected: self.sequence[0].arity(),
                found: param_count,
            });
        }
        Ok(())
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", Formalize::formalize(self))
    }
}

impl Debug for Term {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> FmtResult {
        write!(f, "{}", Formalize::formalize(self))
    }
}
