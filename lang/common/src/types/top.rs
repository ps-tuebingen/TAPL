use super::Type;
use crate::{
    check::{to_subty_err, Kindcheck, Subtypecheck},
    errors::{Error, ErrorKind},
    eval::Normalize,
    kinds::Kind,
    language::LanguageType,
    subst::SubstType,
    TypeVar,
};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Top<Ty>
where
    Ty: Type,
{
    kind: Kind,
    phantom: PhantomData<Ty>,
}

impl<Ty> Top<Ty>
where
    Ty: Type,
{
    pub fn new(knd: Kind) -> Top<Ty> {
        Top {
            kind: knd,
            phantom: PhantomData,
        }
    }

    pub fn new_star() -> Top<Ty> {
        Top {
            kind: Kind::Star,
            phantom: PhantomData,
        }
    }
}

impl<Ty> Type for Top<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Top<Ty>
where
    Ty: Type,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl<Ty> Subtypecheck<Ty> for Top<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;

    fn check_subtype(&self, sup: &Ty, _: &mut Self::Env) -> Result<(), Error> {
        Err(to_subty_err(ErrorKind::Subtype {
            sub: self.to_string(),
            sup: sup.to_string(),
        }))
    }
}

impl<Ty> Kindcheck<Ty> for Top<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;
    fn check_kind(&self, _: &mut Self::Env) -> Result<Kind, Error> {
        Ok(self.kind.clone())
    }
}
impl<Ty> Normalize<Ty> for Top<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    fn normalize(self) -> Ty {
        self.into()
    }
}

impl<Ty> fmt::Display for Top<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Top")
    }
}
