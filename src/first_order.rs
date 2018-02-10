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
                let ty2 = t.type_of(ctx.clone())?;
                if ty1 != &ty2 {
                    return None;
                }
                let mut ctx = s.type_of(ctx)?;
                ctx.push(ty2);
                Some(ctx)
            }
            Compose(ref s1, ref s2) => {
                // TODO: Although this follows the paper, I'm not sure that this is correct.
                let ctx = s2.type_of(ctx)?;
                s1.type_of(ctx)
            }
        }
    }
}

impl TypeCheck for Term {
    type Output = Option<Type>;

    fn type_of(&self, mut ctx: Context) -> Self::Output {
        use self::Term::*;
        match *self {
            Var(n) => ctx.get(n).cloned(),
            Abs(ref ty1, ref t) => {
                ctx.push(ty1.clone());
                let ty2 = t.type_of(ctx)?;
                Some(Type::arr(ty1.clone(), ty2))
            }
            App(ref t1, ref t2) => {
                let ty1 = t1.type_of(ctx.clone())?;
                let (ty11, ty12) = ty1.get_arr()?;
                let ty2 = t2.type_of(ctx)?;
                if ty11 != ty2 { None } else { Some(ty12) }
            }
            Subst(ref t, ref s) => t.type_of(s.type_of(ctx)?),
        }
    }
}

impl Type {
    fn arr(t1: Type, t2: Type) -> Type {
        Type::Arr(Box::new(t1), Box::new(t2))
    }

    fn get_arr(self) -> Option<(Type, Type)> {
        if let Type::Arr(t1, t2) = self {
            Some((*t1, *t2))
        } else {
            None
        }
    }
}

impl Context {
    fn pop(&mut self) -> Option<Type> {
        self.0.pop()
    }

    fn push(&mut self, ty: Type) {
        self.0.push(ty)
    }

    fn get(&mut self, n: usize) -> Option<&Type> {
        self.0.get(n)
    }
}
