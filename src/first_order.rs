#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Var(usize),
    App(Box<Term>, Box<Term>),
    Abs(Type, Box<Term>),
    Subst(Box<Term>, Subst),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Subst {
    Id,
    Shift,
    Cons(Box<Term>, Type, Box<Subst>),
    Compose(Box<Subst>, Box<Subst>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    K,
    Arr(Box<Type>, Box<Type>),
}

pub struct Context(Vec<Type>);

trait TypeCheck {
    type Output;

    fn type_of(&self, ctx: Context) -> Self::Output;
}

impl Context {
    fn pop(&mut self) -> Option<Type> {
        self.0.pop()
    }
}
