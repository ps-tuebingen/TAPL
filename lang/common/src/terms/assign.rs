use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::Error,
    eval::{to_eval_err, Eval, EvalEnvironment, Normalize},
    language::{LanguageTerm, LanguageType, LanguageValue},
    subst::{SubstTerm, SubstType},
    types::Unit as UnitTy,
    values::Unit as UnitVal,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Assign<T>
where
    T: LanguageTerm,
{
    lhs: Box<T>,
    rhs: Box<T>,
}

impl<T> Assign<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, T2>(lhs: T1, rhs: T2) -> Assign<T>
    where
        T1: Into<T>,
        T2: Into<T>,
    {
        Assign {
            lhs: Box::new(lhs.into()),
            rhs: Box::new(rhs.into()),
        }
    }
}

impl<T> Term for Assign<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Assign<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Assign {
            lhs: Box::new(self.lhs.subst(v, t)),
            rhs: Box::new(self.rhs.subst(v, t)),
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Assign<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Assign {
            lhs: Box::new(self.lhs.subst_type(v, ty)),
            rhs: Box::new(self.rhs.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for Assign<T>
where
    T: LanguageTerm,
    UnitTy<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let lhs_ty = self
            .lhs
            .check(&mut &mut env.clone().clone())?
            .normalize(&mut env.clone());
        lhs_ty
            .check_kind(&mut env.clone())?
            .into_star()
            .map_err(to_check_err)?;
        let lhs_ref = lhs_ty.into_ref().map_err(to_check_err)?;

        let rhs_ty = self
            .rhs
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        rhs_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        lhs_ref.ty.check_equal(&rhs_ty).map_err(to_check_err)?;
        Ok(UnitTy::new().into())
    }
}

impl<T> Eval for Assign<T>
where
    T: LanguageTerm,
    UnitVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let lhs_val = self.lhs.eval(env)?;
        let lhs_loc = lhs_val.into_loc().map_err(to_eval_err)?;
        let rhs_val = self.rhs.eval(env)?;
        env.save_location(lhs_loc.loc, rhs_val);
        Ok(UnitVal::new().into())
    }
}

impl<T> fmt::Display for Assign<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) := {}", self.lhs, self.rhs)
    }
}
