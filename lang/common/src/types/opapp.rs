use super::Type;
use crate::{
    check::{to_kind_err, Kindcheck, Subtypecheck},
    errors::{Error, ErrorKind},
    eval::Normalize,
    kinds::Kind,
    language::LanguageType,
    subst::SubstType,
    TypeVar,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpApp<Ty>
where
    Ty: Type,
{
    fun: Box<Ty>,
    arg: Box<Ty>,
}

impl<Ty> OpApp<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1, Ty2>(fun: Ty1, arg: Ty2) -> OpApp<Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        OpApp {
            fun: Box::new(fun.into()),
            arg: Box::new(arg.into()),
        }
    }
}

impl<Ty> Type for OpApp<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for OpApp<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        OpApp {
            fun: Box::new(self.fun.subst_type(v, ty)),
            arg: Box::new(self.arg.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> Subtypecheck<Ty> for OpApp<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        println!("checking subtype {self}<:{sup}");
        let self_norm = self.clone().normalize(env);
        let sup_norm = sup.clone().normalize(env);
        self_norm.check_subtype(&sup_norm, env)
    }
}

impl<Ty> Kindcheck<Ty> for OpApp<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        let fun_kind = self.fun.check_kind(&mut env.clone())?;
        let (fun_from, fun_to) = fun_kind.into_arrow().map_err(to_kind_err)?;
        let arg_kind = self.arg.check_kind(env)?;
        println!("comparing op app {fun_from} == {arg_kind}");
        if fun_from == arg_kind {
            Ok(fun_to)
        } else {
            Err(to_kind_err(ErrorKind::KindMismatch {
                found: arg_kind.to_string(),
                expected: fun_from.to_string(),
            }))
        }
    }
}

impl<Ty> Normalize<Ty> for OpApp<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        let fun_norm = self.fun.normalize(env);
        let arg_norm = self.arg.normalize(env);

        if let Ok(oplam) = fun_norm.clone().into_oplambda() {
            oplam.body.subst_type(&oplam.var, &arg_norm).normalize(env)
        } else if let Ok(oplam) = fun_norm.clone().into_oplambdasub() {
            oplam.body.subst_type(&oplam.var, &arg_norm).normalize(env)
        } else {
            OpApp {
                fun: Box::new(fun_norm),
                arg: Box::new(arg_norm),
            }
            .into()
        }
    }
}

impl<Ty> fmt::Display for OpApp<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}[{}])", self.fun, self.arg)
    }
}
