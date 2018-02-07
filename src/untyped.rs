#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Var(usize, usize),
    App(Box<Term>, Box<Term>),
    Abs(Box<Term>),
    Subst(Box<Term>, Subst),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Subst {
    Id,
    Shift,
    Cons(Box<Term>, Box<Subst>),
    Compose(Box<Subst>, Box<Subst>),
}
