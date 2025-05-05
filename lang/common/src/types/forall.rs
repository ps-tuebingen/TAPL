use super::Type;
use crate::{
    check::{CheckEnvironment, Kindcheck},
    errors::Error,
    eval::Normalize,
    kinds::Kind,
    language::LanguageType,
    subst::SubstType,
    TypeVar,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Forall<Ty>
where
    Ty: Type,
{
    pub var: TypeVar,
    pub kind: Kind,
    pub ty: Box<Ty>,
}

impl<Ty> Forall<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(v: &str, knd: Kind, ty: Ty1) -> Forall<Ty>
    where
        Ty1: Into<Ty>,
    {
        Forall {
            var: v.to_owned(),
            kind: knd,
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for Forall<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Forall<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        if *v == self.var {
            self.into()
        } else {
            Forall {
                var: self.var,
                kind: self.kind,
                ty: Box::new(self.ty.subst_type(v, ty)),
            }
            .into()
        }
    }
}

impl<Ty> Kindcheck<Ty> for Forall<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        let ty_kind = self.ty.check_kind(env)?;
        Ok(ty_kind)
    }
}

impl<Ty> Normalize<Ty> for Forall<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Env = <Ty as Normalize<Ty>>::Env;
    fn normalize(self, env: &mut Self::Env) -> Ty {
        env.add_tyvar_kind(self.var.clone(), self.kind.clone());
        let ty_norm = self.ty.normalize(env);
        Forall {
            var: self.var,
            kind: self.kind,
            ty: Box::new(ty_norm),
        }
        .into()
    }
}

impl<Ty> fmt::Display for Forall<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "forall {}::{}.{}", self.var, self.kind, self.ty)
    }
}
