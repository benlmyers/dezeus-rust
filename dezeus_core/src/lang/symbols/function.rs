use super::*;

struct Function {
    notation: String,
    arity: i8,
}

impl Symbol for Function {
    fn formal(&self) -> String {
        // Get a new string of the form "NOTATION(a1, a2, .., an) where n is the arity"
        let mut notation = self.notation.clone();
        notation.push('(');
        for i in 0..self.arity {
            notation.push_str(&format!("a{}", i));
            if i < self.arity - 1 {
                notation.push_str(", ");
            }
        }
        notation.push(')');
        notation
    }
}
