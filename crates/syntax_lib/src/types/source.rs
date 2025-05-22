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
pub struct Source<Ty>
where
    Ty: Type,
{
    pub ty: Box<Ty>,
}

impl<Ty> Source<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(ty: Ty1) -> Source<Ty>
    where
        Ty1: Into<Ty>,
    {
        Source {
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for Source<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Source<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Source {
            ty: Box::new(self.ty.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> Subtypecheck<Ty> for Source<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_src = sup.clone().into_source().map_err(to_subty_err)?;
        self.ty.check_subtype(&(*sup_src.ty), env)
    }
}

impl<Ty> fmt::Display for Source<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Source({})", self.ty)
    }
}
