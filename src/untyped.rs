#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Var,
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

impl Term {
    pub fn beta(self, t: Term) -> Term {
        Term::Subst(Box::new(self), Subst::cons(t, Subst::Id))
    }
}

impl Subst {
    fn cons(t: Term, s: Subst) -> Self {
        Subst::Cons(Box::new(t), Box::new(s))
    }
}
