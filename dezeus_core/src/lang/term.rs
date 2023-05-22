use super::language::Language;
use super::symbol::*;

pub struct Term {
    language: Language,
    sequence: Vec<Symbol>,
}

impl Term {
    pub fn new(language: Language, sequence: Vec<Symbol>) -> Self {
        Self { language, sequence }
    }
    pub fn sequence(&self) -> Vec<Symbol> {
        self.sequence.clone()
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