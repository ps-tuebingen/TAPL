use super::Term;
use crate::{
    check::{to_check_err, Typecheck},
    errors::Error,
    eval::{to_eval_err, Eval},
    language::{LanguageTerm, LanguageType, LanguageValue},
    subst::{SubstTerm, SubstType},
    types::{Fun, Type},
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct App<T>
where
    T: LanguageTerm,
{
    pub fun: Box<T>,
    pub arg: Box<T>,
}

impl<T> App<T>
where
    T: LanguageTerm,
{
    pub fn new<F: Into<T>, A: Into<T>>(f: F, a: A) -> App<T> {
        App {
            fun: Box::new(f.into()),
            arg: Box::new(a.into()),
        }
    }
}

impl<T> Term for App<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for App<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> Self::Target {
        App {
            fun: Box::new(self.fun.subst(v, t)),
            arg: Box::new(self.arg.subst(v, t)),
        }
        .into()
    }
}
impl<Ty, T> SubstType<Ty> for App<T>
where
    Ty: Type,
    T: LanguageTerm<Type = Ty>,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        App {
            fun: Box::new(self.fun.subst_type(v, ty)),
            arg: Box::new(self.arg.subst_type(v, ty)),
        }
        .into()
    }
}

impl<T> Typecheck for App<T>
where
    T: LanguageTerm,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let fun_ty = self.fun.check(&mut env.clone())?;
        let fun: Fun<<T as LanguageTerm>::Type> = fun_ty.into_fun().map_err(to_check_err)?;
        let arg_ty = self.arg.check(env)?;
        fun.from.check_equal(&arg_ty).map_err(to_check_err)?;
        Ok(*fun.to)
    }
}

impl<T> Eval for App<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as LanguageTerm>::Value;

    fn eval(self, env: &mut <T as Eval>::Env) -> Result<<T as LanguageTerm>::Value, Error> {
        let fun_val = self.fun.eval(env)?;

        let lam = fun_val.into_lambda().map_err(to_eval_err)?;
        let arg_val: <T as LanguageTerm>::Value = self.arg.eval(env)?;
        lam.body.subst(&lam.var, &arg_val.into()).eval(env)
    }
}

impl<T> fmt::Display for App<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) ({})", self.fun, self.arg)
    }
}
