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

#[derive(Clone)]
pub struct Context(Vec<Type>);

trait TypeCheck {
    type Output;

    fn type_of(&self, ctx: Context) -> Self::Output;
}

impl TypeCheck for Subst {
    type Output = Option<Context>;

    fn type_of(&self, mut ctx: Context) -> Self::Output {
        use self::Subst::*;
        match *self {
            Id => Some(ctx),
            Shift => {
                ctx.pop();
                Some(ctx)
            }
            Cons(ref t, ref ty1, ref s) => {
                let ty2 = t.type_of(ctx.clone());
                if ty1 != &ty2 {
                    return None;
                }
                let mut ctx = s.type_of(ctx)?;
                ctx.push(ty2);
                Some(ctx)
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
