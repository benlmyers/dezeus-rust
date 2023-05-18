use super::*;

pub struct Variable {
    pub name: String,
}

impl Variable {
    pub fn new(name: &str) -> Variable {
        Variable {
            name: name.to_string(),
        }
    }
}

impl Symbol for Variable {
    fn formal(&self) -> String {
        self.name.clone()
    }
}
