#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Var(usize),
    App(Box<Term>, Box<Term>),
    Abs(Box<Term>),
    TApp(Box<Term>, Type),
    TAbs(Type, Box<Term>),
    Subst(Box<Term>, Subst),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Subst {
    Id,
    Shift,
    Cons(Box<Term>, Box<Type>, Box<Subst>),
    TCons(Box<Type>, Box<Subst>),
    Compose(Box<Subst>, Box<Subst>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Var(usize),
    Arr(Box<Type>, Box<Type>),
    Abs(Box<Type>),
    Subst(Box<Type>, Subst),
}

#[derive(Clone)]
pub struct Context(Vec<Binding>);

#[derive(Clone)]
enum Binding {
    Term(Type),
    Type,
}

trait Valid {
    fn is_valid(&self) -> bool;
}

impl Valid for Binding {
    fn is_valid(&self) -> bool {
        use self::Binding::*;
        match *self {
            Term(ref ty) => ty.is_valid(),
            Type => true,
        }
    }
}
