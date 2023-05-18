use super::symbol::*;

pub struct Term {
    sequence: Vec<Symbol>,
}

impl Term {
    pub fn new(sequence: Vec<Symbol>) -> Self {
        Self { sequence }
    }
    pub fn sequence(&self) -> Vec<Symbol> {
        self.sequence.clone()
    }
}
