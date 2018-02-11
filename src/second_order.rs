use std::ops::{Deref, DerefMut};

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

impl Type {
    fn is_valid(&self, ctx: &Context) -> bool {
        use self::Type::*;
        use self::Subst::*;
        match *self {
            Var(n) => ctx.get(n).is_some(),
            Arr(ref ty1, ref ty2) => {
                if !ty1.is_valid(ctx) {
                    return false;
                }
                let mut ctx = ctx.clone();
                ctx.push(Binding::Term(*ty1.clone()));
                ty2.is_valid(&ctx)
            }
            Abs(ref ty) => {
                let mut ctx = ctx.clone();
                ctx.push(Binding::Type);
                ty.is_valid(&ctx)
            }
            Subst(ref ty, ref s) => {
                match ty {
                    Abs(ref ty1) => {
                        let mut ctx = ctx.clone();
                        ctx.push(Binding::Type);
                        Subst(ty1, TCons(Var(0), Compose(s, Shift))).is_valid(&ctx)
                    }
                    _ => unimplemented!(),
                }
            }
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

impl DerefMut for Context {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
