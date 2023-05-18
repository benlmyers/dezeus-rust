use super::*;

pub struct Equals {}

impl Symbol for Equals {
    fn notation(&self) -> String {
        "=".to_string()
    }
}
