use super::Term;
use crate::{
    check::{to_check_err, Typecheck},
    errors::{Error, ErrorKind},
    eval::{to_eval_err, Eval},
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
        let fun_ty = self.fun.check(env)?;
        if let Ok(_forall) = fun_ty.clone().into_forall() {
            todo!()
        } else if let Ok(_forall) = fun_ty.clone().into_forall_bounded() {
            todo!()
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
        write!(f, "({})[{}]", self.fun, self.arg)
    }
}
