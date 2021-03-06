//! The untyped λσ-calculus.

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    Var(usize),
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

pub enum Whnf {
    Abs(Term),
    App(usize, Vec<Term>),
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

    /// Evaluates a term in a global environment `s`. The result is in its weak normal head form.
    pub fn whnf(self, s: Subst) -> (Whnf, Subst) {
        use self::Term::*;
        use self::Subst::*;
        use self::Subst;
        match self {
            Abs(t) => (Whnf::Abs(*t), s),
            App(t1, t2) => {
                let (t, s1) = t1.whnf(s.clone());
                match t {
                    Whnf::Abs(t) => t.whnf(Subst::cons(Subst(t2, s), s1)),
                    Whnf::App(n, mut ts) => {
                        ts.push(Subst(t2, s));
                        (Whnf::App(n, ts), Id)
                    }
                }
            }
            Var(n) => {
                match s {
                    Id => (Whnf::num(n), s),
                    Shift => (Whnf::num(n + 1), Id),
                    Cons(t, s) => {
                        match n {
                            0 => {
                                let t = *t;
                                match t {
                                    Subst(a, s) => a.whnf(s),
                                    _ => (Whnf::num(n), Subst::cons(t, *s)),
                                }
                            }
                            _ => Var(n - 1).whnf(*s),
                        }
                    }
                    Compose(s1, s2) => Term::subst(self, *s1).whnf(*s2),
                }
            }
            Subst(t, s0) => {
                match *t {
                    Var(n) => {
                        match s0 {
                            Id => t.whnf(s),
                            Shift => Var(n + 1).whnf(s),
                            Cons(t, s0) => {
                                match n {
                                    0 => t.whnf(s),
                                    _ => Term::subst(Var(n - 1), *s0).whnf(s),
                                }
                            }
                            Compose(s1, s2) => Subst(t, *s1).whnf(Subst::compose(*s2, s)),
                        }
                    }
                    _ => t.whnf(Subst::compose(s0, s)),
                }
            }
        }
    }
}

impl Subst {
    fn cons(t: Term, s: Subst) -> Self {
        Subst::Cons(Box::new(t), Box::new(s))
    }

    fn compose(s1: Subst, s2: Subst) -> Self {
        Subst::Compose(Box::new(s1), Box::new(s2))
    }
}

impl Whnf {
    fn num(n: usize) -> Self {
        Whnf::App(n, vec![])
    }
}
