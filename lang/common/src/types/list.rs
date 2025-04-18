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
pub struct List<Ty>
where
    Ty: Type,
{
    pub ty: Box<Ty>,
}

impl<Ty> List<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(ty: Ty1) -> List<Ty>
    where
        Ty1: Into<Ty>,
    {
        List {
            ty: Box::new(ty.into()),
        }
    }
}

impl<Ty> Type for List<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for List<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        List {
            ty: Box::new(self.ty.subst_type(v, ty)),
        }
        .into()
    }
}
impl<Ty> Subtypecheck<Ty> for List<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if let Ok(_) = sup.clone().into_top() {
            return Ok(());
        }

        let sup_list = sup.clone().into_list().map_err(to_subty_err)?;
        self.ty.check_subtype(&(*sup_list.ty), env)
    }

    fn check_supertype(&self, sub: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if let Ok(_) = sub.clone().into_bot() {
            return Ok(());
        }

        let sub_list = sub.clone().into_list().map_err(to_subty_err)?;
        self.ty.check_supertype(&(*sub_list.ty), env)
    }
}

impl<Ty> fmt::Display for List<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "List[{}]", self.ty)
    }
}
