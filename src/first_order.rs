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

impl TypeCheck for Subst {
    type Output = Context;

    fn type_of(&self, ctx: Context) -> Self::Output {
        use self::Subst::*;
        match *self {
            Id => ctx,
            Shift => {
                ctx.pop();
                ctx
            }
            Cons(ref t, ref ty, ref s) => {
                unimplemented!();
            }
            Compose(..) => {
                unimplemented!();
            }
        }
    }
}

impl TypeCheck for Term {
    type Output = Type;

    fn type_of(&self, mut ctx: Context) -> Self::Output {
        unimplemented!();
    }
}

impl Context {
    fn pop(&mut self) -> Option<Type> {
        self.0.pop()
    }

    fn push(&mut self, ty: Type) {
        self.0.push(ty)
    }
}
