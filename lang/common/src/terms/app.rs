use super::{Lambda, Term};
use crate::{
    check::{to_check_err, Kindcheck, Subtypecheck, Typecheck},
    errors::Error,
    eval::{to_eval_err, Eval},
    language::{LanguageTerm, LanguageType, LanguageValue},
    subst::{SubstTerm, SubstType},
    types::{Fun, Type, Unit as UnitTy},
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

    pub fn seq<T1, T2>(t1: T1, t2: T2) -> App<T>
    where
        T1: Into<T>,
        T2: Into<T>,
        Lambda<T>: Into<T>,
        UnitTy<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
    {
        App {
            fun: Box::new(Lambda::new("_", UnitTy::new(), t2).into()),
            arg: Box::new(t1.into()),
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
        fun_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        let fun: Fun<<T as LanguageTerm>::Type> = fun_ty.into_fun().map_err(to_check_err)?;
        let arg_ty = self.arg.check(env)?;
        arg_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        arg_ty.check_subtype(&(*fun.from), env)?;
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
        println!("checking app");
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
