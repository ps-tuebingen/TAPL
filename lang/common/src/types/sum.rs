use super::Type;
use crate::{
    check::{to_kind_err, Kindcheck},
    errors::Error,
    kinds::Kind,
    language::LanguageType,
    subst::SubstType,
    TypeVar,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sum<Ty>
where
    Ty: Type,
{
    pub left: Box<Ty>,
    pub right: Box<Ty>,
}

impl<Ty> Sum<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1, Ty2>(l: Ty1, r: Ty2) -> Sum<Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        Sum {
            left: Box::new(l.into()),
            right: Box::new(r.into()),
        }
    }
}

impl<Ty> Type for Sum<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Sum<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Sum {
            left: Box::new(self.left.subst_type(v, ty)),
            right: Box::new(self.right.subst_type(v, ty)),
        }
        .into()
    }
}

impl<Ty> Kindcheck<Ty> for Sum<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;
    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        let left_kind = self.left.check_kind(env)?;
        let right_kind = self.right.check_kind(env)?;
        left_kind.check_equal(&right_kind).map_err(to_kind_err)?;
        Ok(left_kind)
    }
}

impl<Ty> fmt::Display for Sum<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}) + ({})", self.left, self.right)
    }
}
