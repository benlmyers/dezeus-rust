use super::*;

pub struct Connective {
    pub notation: String,
}

impl Connective {
    pub fn new(notation: &str) -> Connective {
        Connective {
            notation: notation.to_string(),
        }
    }
}

impl Symbol for Connective {
    fn formal(&self) -> String {
        self.notation.clone()
    }
}
