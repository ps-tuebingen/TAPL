use super::Term;
use crate::{
    check::{CheckEnvironment, Typecheck},
    errors::Error,
    subst::{SubstTerm, SubstType},
    types::Type,
    TypeVar, Var,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Exception<T, Ty>
where
    Ty: Type,
    T: Term,
{
    ty: Ty,
    phantom: PhantomData<T>,
}

impl<T, Ty> Exception<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<Typ: Into<Ty>>(ty: Typ) -> Exception<T, Ty> {
        Exception {
            ty: ty.into(),
            phantom: PhantomData,
        }
    }
}

impl<T, Ty> Term for Exception<T, Ty>
where
    Ty: Type,
    T: Term,
{
}

impl<T, Ty> SubstTerm<T> for Exception<T, Ty>
where
    T: Term,
    Self: Into<T>,
    Ty: Type,
{
    type Target = T;
    fn subst(self, _: &Var, _: &T) -> T {
        self.into()
    }
}

impl<Ty, T> SubstType<Ty> for Exception<T, Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    T: Term,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Exception {
            ty: self.ty.subst_type(v, ty),
            phantom: PhantomData,
        }
        .into()
    }
}

impl<Env, T, Ty> Typecheck<Env, Ty> for Exception<T, Ty>
where
    Env: CheckEnvironment<Ty>,
    Ty: Type,
    T: Term,
{
    fn check_start(&self) -> Result<Ty, Error> {
        self.check(&mut Env::default())
    }

    fn check(&self, _: &mut Env) -> Result<Ty, Error> {
        Ok(self.ty.clone())
    }
}

impl<T, Ty> fmt::Display for Exception<T, Ty>
where
    Ty: Type,
    T: Term,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error[{}]", self.ty)
    }
}
