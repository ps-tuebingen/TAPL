use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::Error,
    eval::{Eval, Normalize},
    language::{LanguageTerm, LanguageType},
    subst::{SubstTerm, SubstType},
    types::List,
    values::Cons as ConsVal,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cons<T>
where
    T: LanguageTerm,
{
    head: Box<T>,
    tail: Box<T>,
    ty: <T as LanguageTerm>::Type,
}

impl<T> Cons<T>
where
    T: LanguageTerm,
{
    pub fn new<H, Tl, Typ>(h: H, tl: Tl, ty: Typ) -> Cons<T>
    where
        H: Into<T>,
        Tl: Into<T>,
        Typ: Into<<T as LanguageTerm>::Type>,
    {
        Cons {
            head: Box::new(h.into()),
            tail: Box::new(tl.into()),
            ty: ty.into(),
        }
    }
}

impl<T> Term for Cons<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Cons<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Cons {
            head: Box::new(self.head.subst(v, t)),
            tail: Box::new(self.tail.subst(v, t)),
            ty: self.ty,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Cons<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Cons {
            head: Box::new(self.head.subst_type(v, ty)),
            tail: Box::new(self.tail.subst_type(v, ty)),
            ty: self.ty.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> Typecheck for Cons<T>
where
    T: LanguageTerm,
    List<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        let hd_ty = self
            .head
            .check(&mut &mut env.clone().clone())?
            .normalize(&mut env.clone());
        hd_ty.check_equal(&ty_norm).map_err(to_check_err)?;
        hd_ty
            .check_kind(&mut env.clone())?
            .into_star()
            .map_err(to_check_err)?;

        let tl_ty = self
            .tail
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        tl_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        let list_ty: Self::Type = List::new(ty_norm).into();
        tl_ty.check_equal(&list_ty).map_err(to_check_err)?;
        Ok(list_ty)
    }
}

impl<T> Eval for Cons<T>
where
    T: LanguageTerm,
    ConsVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let hd_val = self.head.eval(env)?;
        let tail_val = self.tail.eval(env)?;
        Ok(ConsVal::<T>::new(hd_val, tail_val, self.ty).into())
    }
}

impl<T> fmt::Display for Cons<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cons[{}]({},{})", self.ty, self.head, self.tail)
    }
}
