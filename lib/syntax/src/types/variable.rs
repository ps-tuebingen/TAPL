use super::Type;
use crate::{TypeVar, language::Language, subst::SubstType};
use std::{fmt, marker::PhantomData};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeVariable<Lang>
where
    Lang: Language,
{
    pub v: TypeVar,
    phantom: PhantomData<Lang>,
}

impl<Lang> TypeVariable<Lang>
where
    Lang: Language,
{
    pub fn new(v: &str) -> TypeVariable<Lang> {
        TypeVariable {
            v: v.to_owned(),
            phantom: PhantomData,
        }
    }
}

impl<Lang> Type for TypeVariable<Lang> where Lang: Language {}

impl<Lang> SubstType for TypeVariable<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Target = Lang::Type;
    type Lang = Lang;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        if *v == self.v {
            ty.clone()
        } else {
            self.into()
        }
    }
}

impl<Lang> fmt::Display for TypeVariable<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}
