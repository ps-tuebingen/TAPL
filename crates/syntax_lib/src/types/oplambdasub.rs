use super::Type;
use crate::{
    check::{to_subty_err, CheckEnvironment, Kindcheck, Subtypecheck},
    errors::Error,
    eval::Normalize,
    kinds::Kind,
    language::LanguageType,
    subst::SubstType,
    types::TypeVariable,
    TypeVar,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpLambdaSub<Ty>
where
    Ty: Type,
{
    pub var: TypeVar,
    pub sup: Box<Ty>,
    pub body: Box<Ty>,
}

impl<Ty> OpLambdaSub<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1, Ty2>(var: &str, sup: Ty1, ty: Ty2) -> OpLambdaSub<Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        OpLambdaSub {
            var: var.to_owned(),
            sup: Box::new(sup.into()),
            body: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for OpLambdaSub<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for OpLambdaSub<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        let sup_subst = self.sup.subst_type(v, ty);
        if *v == self.var {
            OpLambdaSub {
                var: self.var,
                sup: Box::new(sup_subst),
                body: self.body,
            }
            .into()
        } else {
            OpLambdaSub {
                var: self.var,
                sup: Box::new(sup_subst),
                body: Box::new(self.body.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<Ty> Subtypecheck<Ty> for OpLambdaSub<Ty>
where
    Ty: LanguageType,
    TypeVariable<Ty>: Into<Ty>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        let sup_norm = sup.clone().normalize(env);
        let self_sup_norm = self.sup.clone().normalize(env);
        let sup_op = sup_norm.into_oplambdasub().map_err(to_subty_err)?;
        sup_op
            .sup
            .check_equal(&self_sup_norm)
            .map_err(to_subty_err)?;
        env.add_tyvar_super(self.var.clone(), self_sup_norm);

        self.body.check_subtype(
            &sup_op
                .body
                .subst_type(&sup_op.var, &(TypeVariable::new(&self.var).into())),
            env,
        )
    }
}

impl<Ty> Kindcheck<Ty> for OpLambdaSub<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        let sup_kind = self.sup.check_kind(env)?;
        env.add_tyvar_kind(self.var.clone(), sup_kind.clone());
        let body_kind = self.body.check_kind(env)?;
        Ok(Kind::Arrow(Box::new(sup_kind), Box::new(body_kind)))
    }
}

impl<Ty> Normalize<Ty> for OpLambdaSub<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        let body_norm = self.body.normalize(env);
        OpLambdaSub {
            var: self.var,
            sup: self.sup,
            body: Box::new(body_norm),
        }
        .into()
    }
}

impl<Ty> fmt::Display for OpLambdaSub<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}<:{}.{}", self.var, self.sup, self.body)
    }
}
