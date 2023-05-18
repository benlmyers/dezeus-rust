use super::*;

pub struct Constant {
    pub notation: String,
}

impl Constant {
    pub fn new(notation: &str) -> Constant {
        Constant {
            notation: notation.to_string(),
        }
    }
}

impl Symbol for Constant {
    fn formal(&self) -> String {
        self.notation.clone()
    }
}
