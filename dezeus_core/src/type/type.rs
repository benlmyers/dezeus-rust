pub struct Type<T> {
    layer: i8,
}

impl GetType for Type {
    fn get_type(&self) -> Type {
        Type {
            layer: self.layer + 1,
        }
    }
}
