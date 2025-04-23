use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::Error,
    eval::Eval,
    language::{LanguageTerm, LanguageType},
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Ascribe<T>
where
    T: LanguageTerm,
{
    pub term: Box<T>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Ascribe<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, Ty1>(t: T1, ty: Ty1) -> Ascribe<T>
    where
        T1: Into<T>,
        Ty1: Into<<T as LanguageTerm>::Type>,
    {
        Ascribe {
            term: Box::new(t.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Ascribe<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Ascribe<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Ascribe {
            term: Box::new(self.term.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Ascribe<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Ascribe {
            term: Box::new(self.term.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> Eval for Ascribe<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        self.term.eval(env)
    }
}

impl<T> Typecheck for Ascribe<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;
    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let t_ty = self.term.check(env)?;
        let t_kind = t_ty.check_kind(env)?;
        let ty_kind = self.ty.check_kind(env)?;
        t_kind.check_equal(&ty_kind).map_err(to_check_err)?;
        self.ty.check_equal(&t_ty).map_err(to_check_err)?;
        Ok(self.ty.clone())
    }
}

impl<T> fmt::Display for Ascribe<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} as {})", self.term, self.ty)
    }
}
