use crate::{TypeVar, kinds::Kind, subst::SubstType, types::Type};
use std::{fmt, marker::PhantomData};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bot<Ty>
where
    Ty: Type,
{
    pub kind: Kind,
    phantom: PhantomData<Ty>,
}

impl<Ty> Bot<Ty>
where
    Ty: Type,
{
    pub fn new() -> Bot<Ty> {
        Bot {
            kind: Kind::Star,
            phantom: PhantomData,
        }
    }
}

impl<Ty> Default for Bot<Ty>
where
    Ty: Type,
{
    fn default() -> Bot<Ty> {
        Bot::new()
    }
}

impl<Ty> Type for Bot<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Bot<Ty>
where
    Ty: Type,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, _: &TypeVar, _: &Ty) -> Self::Target {
        self.into()
    }
}

impl<Ty> fmt::Display for Bot<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Bot")
    }
}
