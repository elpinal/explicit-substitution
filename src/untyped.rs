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

    fn whnf(self, s: Subst) -> (Self, Subst) {
        use self::Term::*;
        use self::Subst::*;
        use self::Subst;
        match (self, s) {
            (Abs(..), _) => (self, s),
            (App(t1, t2), s) => {
                let (t, s1) = t1.whnf(s);
                if let Abs(t) = t {
                    t.whnf(Subst::cons(Subst(t2, s), s1))
                } else {
                    (App(t, Subst(t2, s)), Id)
                }
            }
        }
    }
}

impl Subst {
    fn cons(t: Term, s: Subst) -> Self {
        Subst::Cons(Box::new(t), Box::new(s))
    }
}
