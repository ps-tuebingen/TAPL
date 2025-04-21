use super::Term;
use crate::{
    check::{to_check_err, Typecheck},
    errors::Error,
    eval::{to_eval_err, Eval},
    language::{LanguageTerm, LanguageType, LanguageValue},
    subst::{SubstTerm, SubstType},
    types::Nat,
    values::Num as NumVal,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pred<T>
where
    T: LanguageTerm,
{
    term: Box<T>,
}

impl<T> Pred<T>
where
    T: LanguageTerm,
{
    pub fn new<T1>(t: T1) -> Pred<T>
    where
        T1: Into<T>,
    {
        Pred {
            term: Box::new(t.into()),
        }
    }
}

impl<T> Term for Pred<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Pred<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Pred {
            term: Box::new(self.term.subst(v, t)),
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Pred<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Pred {
            term: Box::new(self.term.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for Pred<T>
where
    T: LanguageTerm,
    Nat<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let inner_ty = self.term.check(env)?;
        let nat = inner_ty.into_nat().map_err(to_check_err)?;
        Ok(nat.into())
    }
}

impl<T> Eval for Pred<T>
where
    T: LanguageTerm,
    NumVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let val = self.term.eval(env)?;
        let num = val.into_num().map_err(to_eval_err)?;
        Ok(NumVal::new(num.num - 1).into())
    }
}

impl<T> fmt::Display for Pred<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pred({})", self.term)
    }
}
