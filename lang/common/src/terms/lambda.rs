use super::Term;
use crate::{
    check::{CheckEnvironment, Typecheck},
    errors::Error,
    subst::{SubstTerm, SubstType},
    types::Fun,
    types::Type,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub var: Var,
    pub annot: Ty,
    pub body: Box<T>,
}

impl<T, Ty> Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<A: Into<Ty>, B: Into<T>>(v: &str, a: A, b: B) -> Lambda<T, Ty> {
        Lambda {
            var: v.to_owned(),
            annot: a.into(),
            body: Box::new(b.into()),
        }
    }
}

impl<'a, Env, T, Ty> Typecheck<Env, Ty> for Lambda<T, Ty>
where
    Env: CheckEnvironment<Ty>,
    Ty: Type,
    T: Term + Typecheck<Env, Ty>,
    Fun<Ty>: Into<Ty>,
{
    fn check_start(&self) -> Result<Ty, Error> {
        self.check(&mut Env::default())
    }

    fn check(&self, env: &mut Env) -> Result<Ty, Error> {
        env.add_var(self.var.clone(), self.annot.clone());
        let body_ty = self.body.check(env)?;
        Ok(Fun::new(self.annot.clone(), body_ty).into())
    }
}

impl<T, Ty> SubstTerm<T> for Lambda<T, Ty>
where
    T: Term + SubstTerm<T, Target = T>,
    Self: Into<T>,
    Ty: Type,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        if *v == self.var {
            self.into()
        } else {
            Lambda {
                var: self.var,
                annot: self.annot,
                body: Box::new(self.body.subst(v, t)),
            }
            .into()
        }
    }
}

impl<T, Ty> SubstType<Ty> for Lambda<T, Ty>
where
    T: Term + SubstType<Ty, Target = T>,
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Lambda {
            var: self.var,
            annot: self.annot.subst_type(v, ty),
            body: Box::new(self.body.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T, Ty> Term for Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
}

impl<T, Ty> fmt::Display for Lambda<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}:{}.{}", self.var, self.annot, self.body)
    }
}
