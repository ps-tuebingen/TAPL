use super::Value;
use crate::{language::Language, terms::Exception as ExceptionT};
use std::{fmt, marker::PhantomData};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Exception<Lang>
where
    Lang: Language,
{
    pub ty: Lang::Type,
    phantom: PhantomData<Lang::Term>,
}

impl<Lang> Exception<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1>(ty: Ty1) -> Exception<Lang>
    where
        Ty1: Into<Lang::Type>,
    {
        Exception {
            ty: ty.into(),
            phantom: PhantomData,
        }
    }
}

impl<Lang> Value for Exception<Lang>
where
    Lang: Language,
    ExceptionT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;
    type Term = ExceptionT<Lang>;
}

impl<Lang> From<Exception<Lang>> for ExceptionT<Lang>
where
    Lang: Language,
{
    fn from(ex: Exception<Lang>) -> ExceptionT<Lang> {
        ExceptionT::new(ex.ty)
    }
}

impl<Lang> fmt::Display for Exception<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error[{}]", self.ty)
    }
}
