use super::Type;
use crate::{
    check::{to_subty_err, CheckEnvironment, Subtypecheck},
    errors::{Error, ErrorKind},
    kinds::Kind,
    language::LanguageType,
    subst::SubstType,
    types::TypeVariable,
    TypeVar,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OpLambda<Ty>
where
    Ty: Type,
{
    var: TypeVar,
    pub annot: Kind,
    body: Box<Ty>,
}

impl<Ty> OpLambda<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(var: &str, knd: Kind, ty: Ty1) -> OpLambda<Ty>
    where
        Ty1: Into<Ty>,
    {
        OpLambda {
            var: var.to_owned(),
            annot: knd,
            body: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for OpLambda<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for OpLambda<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        if *v == self.var {
            self.into()
        } else {
            OpLambda {
                var: self.var,
                annot: self.annot,
                body: Box::new(self.body.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<Ty> Subtypecheck<Ty> for OpLambda<Ty>
where
    Ty: LanguageType,
    TypeVariable: Into<Ty>,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        let sup_op = sup.clone().into_oplambda().map_err(to_subty_err)?;
        if sup_op.annot != self.annot {
            return Err(to_subty_err(ErrorKind::KindMismatch {
                found: sup_op.annot.to_string(),
                expected: self.annot.to_string(),
            }));
        }
        env.add_tyvar_kind(self.var.clone(), self.annot.clone());
        self.body.check_subtype(
            &sup_op
                .body
                .subst_type(&sup_op.var, &(TypeVariable::new(&self.var).into())),
            env,
        )
    }

    fn check_supertype(&self, sub: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        let sub_op = sub.clone().into_oplambda().map_err(to_subty_err)?;
        if sub_op.annot != self.annot {
            return Err(to_subty_err(ErrorKind::KindMismatch {
                found: sub_op.annot.to_string(),
                expected: self.annot.to_string(),
            }));
        }
        env.add_tyvar_kind(self.var.clone(), self.annot.clone());
        self.body.check_supertype(
            &sub_op
                .body
                .subst_type(&sub_op.var, &(TypeVariable::new(&self.var).into())),
            env,
        )
    }
}

impl<Ty> fmt::Display for OpLambda<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\\{}::{}.{}", self.var, self.annot, self.body)
    }
}
