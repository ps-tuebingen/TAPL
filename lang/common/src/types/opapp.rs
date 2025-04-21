use super::Type;
use crate::{
    check::{to_subty_err, Subtypecheck},
    errors::Error,
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
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        let sup_app = sup.clone().into_opapp().map_err(to_subty_err)?;
        self.fun.check_subtype(&sup_app.fun, &mut env.clone())?;
        self.arg.check_subtype(&sup_app.arg, env)
    }
    fn check_supertype(&self, sub: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        let sub_app = sub.clone().into_opapp().map_err(to_subty_err)?;
        self.fun.check_supertype(&sub_app.fun, &mut env.clone())?;
        self.arg.check_supertype(&sub_app.arg, env)
    }
}

impl<Ty> fmt::Display for OpApp<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({})[{}]", self.fun, self.arg)
    }
}
