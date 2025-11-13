use super::Type;
use crate::{TypeVar, language::Language, subst::SubstType};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tuple<Lang>
where
    Lang: Language,
{
    pub tys: Vec<Lang::Type>,
}

impl<Lang> Tuple<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1>(tys: Vec<Ty1>) -> Self
    where
        Ty1: Into<Lang::Type>,
    {
        Self {
            tys: tys.into_iter().map(std::convert::Into::into).collect(),
        }
    }
}

impl<Lang> Type for Tuple<Lang> where Lang: Language {}

impl<Lang> SubstType for Tuple<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &<Lang as Language>::Type) -> Self::Target {
        Self {
            tys: self
                .tys
                .into_iter()
                .map(|ty1| ty1.subst_type(v, ty))
                .collect(),
        }
    }
}

impl<Lang> fmt::Display for Tuple<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut tys: Vec<String> = self.tys.iter().map(std::string::ToString::to_string).collect();
        tys.sort();
        write!(f, "({})", tys.join(", "))
    }
}
