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
pub struct Reference<Ty>
where
    Ty: Type,
{
    pub ty: Box<Ty>,
}

impl<Ty> Reference<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(ty: Ty1) -> Reference<Ty>
    where
        Ty1: Into<Ty>,
    {
        Reference {
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for Reference<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Reference<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Reference {
            ty: Box::new(self.ty.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> Subtypecheck<Ty> for Reference<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        if let Ok(src) = sup.clone().into_source() {
            self.ty.check_subtype(&(*src.ty), env)
        } else if let Ok(sink) = sup.clone().into_sink() {
            sink.ty.check_subtype(&(*sink.ty), env)
        } else {
            let sup_ref = sup.clone().into_ref().map_err(to_subty_err)?;
            sup_ref.ty.check_subtype(&(*self.ty), &mut env.clone())?;
            self.ty.check_subtype(&(*sup_ref.ty), env)
        }
    }
}

impl<Ty> fmt::Display for Reference<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Ref({})", self.ty)
    }
}
