use std::fmt::Result as FmtResult;
use std::fmt::{Debug, Display};

use super::expression::{Error as ExprError, Expression};
use super::symbol::*;

#[derive(Clone, PartialEq)]
pub struct Term {
    expression: Expression,
}

#[derive(PartialEq)]
pub enum Error {
    Expression(ExprError),
    TooShort,
    ExpectedSymbol {
        symbol: Symbol,
        position: usize,
    },
    ExpectedVariant {
        variant: Variant,
        position: usize,
    },
    InvalidSubterm {
        subexpression: Expression,
        cause: Box<Error>,
    },
    InvalidArity {
        expected: i8,
        found: i8,
    },
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> FmtResult {
        match self {
            Error::Expression(cause) => write!(f, "{:?}", cause),
            Error::TooShort => write!(f, "Term is too short."),
            Error::ExpectedSymbol { symbol, position } => {
                write!(
                    f,
                    "Expected symbol \'{}\' at position {}.",
                    symbol, position
                )
            }
            Error::ExpectedVariant { variant, position } => {
                write!(
                    f,
                    "Expected variant {:?} at position {}.",
                    variant, position
                )
            }
            Error::InvalidSubterm {
                subexpression,
                cause,
            } => {
                write!(f, "Invalid subterm {}: {:?}", subexpression, cause)
            }
            Error::InvalidArity { expected, found } => {
                write!(f, "Invalid arity: expected {}, found {}", expected, found)
            }
        }
    }
}

impl Term {
    pub fn new(expression: Expression) -> Result<Self, Error> {
        let term = Term { expression };
        if term.expr().size() == 1 {
            if term.is_variable() {
                return Ok(term);
            }
            if term.is_constant() {
                return Ok(term);
            }
            return Err(Error::ExpectedSymbol {
                symbol: Symbol::left_paren(),
                position: 1,
            });
        }
        let c = term.val_composite();
        if c.is_some() {
            return Err(c.unwrap());
        }
        let s = term.subterms();
        match s {
            Ok(_) => Ok(term),
            Err(e) => Err(e),
        }
    }

    pub fn expr(&self) -> Expression {
        self.expression.clone()
    }

    fn is_variable(&self) -> bool {
        if self.expr().size() != 1 {
            return false;
        }
        self.expr().sequence()[0].variant() == Variant::Variable
    }

    fn is_constant(&self) -> bool {
        if self.expr().size() != 1 {
            return false;
        }
        self.expr().sequence()[0].variant() == Variant::Constant
    }

    fn val_composite(&self) -> Option<Error> {
        if self.expr().size() < 3 {
            return Some(Error::TooShort);
        }
        if self.expr().sequence()[0].variant() != Variant::Function {
            return Some(Error::ExpectedVariant {
                variant: Variant::Function,
                position: 0,
            });
        }
        if self.expr().sequence()[1] != Symbol::left_paren() {
            return Some(Error::ExpectedSymbol {
                symbol: Symbol::left_paren(),
                position: 1,
            });
        }
        if self.expr().sequence()[self.expr().size() - 1] != Symbol::right_paren() {
            return Some(Error::ExpectedSymbol {
                symbol: Symbol::right_paren(),
                position: self.expr().size() - 1,
            });
        }
        None
    }

    fn subterms(&self) -> Result<Vec<Term>, Error> {
        let mut subterms: Vec<Term> = Vec::new();
        if self.is_constant() || self.is_variable() {
            return Ok(subterms);
        }
        let val = self.val_composite();
        if val.is_some() {
            return Err(val.unwrap());
        }
        let mut symbols: Vec<Symbol> = Vec::new();
        for symbol in self.expr().sequence()[2..self.expr().size() - 1].iter() {
            if symbol == &Symbol::comma() || symbol == &Symbol::right_paren() {
                let e = Expression::new(self.expr().language().clone(), symbols.clone());
                if e.is_err() {
                    return Err(Error::Expression(e.unwrap_err()));
                }
                match Term::new(e.unwrap()) {
                    Ok(term) => {
                        symbols.clear();
                        subterms.push(term);
                    }
                    Err(e) => {
                        return Err(Error::InvalidSubterm {
                            subexpression: self.expr(),
                            cause: Box::new(e),
                        })
                    }
                }
            } else {
                symbols.push(symbol.clone());
            }
        }
        Ok(subterms)
    }
}

use super::Formalize;

impl Formalize for Term {
    fn formalize(&self) -> String {
        let mut string = String::new();
        for symbol in self.expr().sequence().iter() {
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
