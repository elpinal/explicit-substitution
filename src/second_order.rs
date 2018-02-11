#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Var(usize),
    Arr(Box<Type>, Box<Type>),
    Abs(Box<Type>),
    Subst(Box<Type>, Subst),
}

#[derive(Clone)]
enum Binding {
    Term(Type),
    Type,
}
