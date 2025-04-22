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
pub struct Sink<Ty>
where
    Ty: Type,
{
    pub ty: Box<Ty>,
}

impl<Ty> Sink<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(ty: Ty1) -> Sink<Ty>
    where
        Ty1: Into<Ty>,
    {
        Sink {
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for Sink<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Sink<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Sink {
            ty: Box::new(self.ty.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> Subtypecheck<Ty> for Sink<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if let Ok(_) = sup.clone().into_top() {
            return Ok(());
        }

        let sup_sink = sup.clone().into_sink().map_err(to_subty_err)?;
        sup_sink.ty.check_subtype(&(*self.ty), env)
    }
}

impl<Ty> fmt::Display for Sink<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sink({})", self.ty)
    }
}
