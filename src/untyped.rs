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
    fn app(t1: Term, t2: Term) -> Term {
        Term::App(Box::new(t1), Box::new(t2))
    }

    fn subst(t: Term, s: Subst) -> Term {
        Term::Subst(Box::new(t), s)
    }

    pub fn beta(self, t: Term) -> Term {
        Term::Subst(Box::new(self), Subst::cons(t, Subst::Id))
    }

    fn whnf(self, s: Subst) -> (Self, Subst) {
        use self::Term::*;
        use self::Subst::*;
        use self::Subst;
        match (self, s) {
            (Abs(..), _) => (self, s),
            (App(t1, t2), _) => {
                let (t, s1) = t1.whnf(s.clone());
                if let Abs(t) = t {
                    t.whnf(Subst::cons(Subst(t2, s), s1))
                } else {
                    (Term::app(t, Subst(t2, s)), Id)
                }
            }
            (Var, _) => {
                match s {
                    Id => (self, s),
                    Shift => (Term::subst(self, Shift), Id),
                    Cons(Subst(a, s), _) => a.whnf(s),
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
