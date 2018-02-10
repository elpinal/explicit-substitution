#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    K,
    Arr(Box<Type>, Box<Type>),
}
