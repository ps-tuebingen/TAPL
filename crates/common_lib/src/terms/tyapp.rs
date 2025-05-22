use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Subtypecheck, Typecheck},
    errors::{Error, ErrorKind},
    eval::{to_eval_err, Eval, Normalize},
    language::{LanguageTerm, LanguageType, LanguageValue},
    subst::{SubstTerm, SubstType},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TyApp<T>
where
    T: LanguageTerm,
{
    fun: Box<T>,
    arg: <T as LanguageTerm>::Type,
}

impl<T> TyApp<T>
where
    T: LanguageTerm,
{
    pub fn new<T1, Typ>(t: T1, ty: Typ) -> TyApp<T>
    where
        T1: Into<T>,
        Typ: Into<<T as LanguageTerm>::Type>,
    {
        TyApp {
            fun: Box::new(t.into()),
            arg: ty.into(),
        }
    }
}

impl<T> Term for TyApp<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for TyApp<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        TyApp {
            fun: Box::new(self.fun.subst(v, t)),
            arg: self.arg,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for TyApp<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        TyApp {
            fun: Box::new(self.fun.subst_type(v, ty)),
            arg: self.arg.subst_type(v, ty),
        }
        .into()
    }
}

impl<T> Eval for TyApp<T>
where
    T: LanguageTerm,
{
    type Value = <T as Eval>::Value;
    type Env = <T as Eval>::Env;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let fun_val = self.fun.eval(env)?;
        if let Ok(tylam) = fun_val.clone().into_tylambda() {
            tylam.term.subst_type(&tylam.var, &self.arg).eval(env)
        } else if let Ok(lamsub) = fun_val.clone().into_lambdasub() {
            lamsub.term.subst_type(&lamsub.var, &self.arg).eval(env)
        } else {
            Err(to_eval_err(ErrorKind::ValueMismatch {
                found: fun_val.to_string(),
                expected: "Type Abstraction".to_owned(),
            }))
        }
    }
}

impl<T> Typecheck for TyApp<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let fun_ty = self
            .fun
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        let arg_norm = self.arg.clone().normalize(&mut env.clone());
        let arg_kind = arg_norm.check_kind(&mut env.clone())?;
        if let Ok(forall) = fun_ty.clone().into_forall() {
            forall.kind.check_equal(&arg_kind).map_err(to_check_err)?;
            Ok(forall.ty.subst_type(&forall.var, &arg_norm))
        } else if let Ok(forall) = fun_ty.clone().into_forall_bounded() {
            let sup_knd = forall.sup_ty.check_kind(&mut env.clone())?;
            sup_knd.check_equal(&arg_kind).map_err(to_check_err)?;
            arg_norm.check_subtype(&forall.sup_ty, env)?;
            Ok(forall.ty.subst_type(&forall.var, &arg_norm))
        } else {
            Err(to_check_err(ErrorKind::TypeMismatch {
                found: fun_ty.to_string(),
                expected: "Universal Type".to_owned(),
            }))
        }
    }
}

impl<T> fmt::Display for TyApp<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(({})[{}])", self.fun, self.arg)
    }
}
