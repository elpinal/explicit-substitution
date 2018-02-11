use std::ops::Deref;

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
pub enum Binding {
    Term(Type),
    Type,
}

trait Valid {
    type Input;

    fn is_valid(&self, &Self::Input) -> bool;
}

impl Valid for Context {
    type Input = ();

    fn is_valid(&self, _: &Self::Input) -> bool {
        self.0.iter().enumerate().all(|(i, b)| b.is_valid(&Context(self[..i].to_vec())))
    }
}

impl Valid for Binding {
    type Input = Context;

    fn is_valid(&self, ctx: &Self::Input) -> bool {
        use self::Binding::*;
        match *self {
            Term(ref ty) => ty.is_valid(ctx),
            Type => true,
        }
    }
}

impl Context {
    fn get(&self, n: usize) -> Option<&Binding> {
        self.0.get(n)
    }
}

impl Deref for Context {
    type Target = Vec<Binding>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
