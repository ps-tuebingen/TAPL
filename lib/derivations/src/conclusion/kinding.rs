use std::fmt;
use syntax::{kinds::Kind, language::Language};

#[derive(Debug)]
pub struct KindingConclusion<Lang>
where
    Lang: Language,
{
    pub ty: Lang::Type,
    pub kind: Kind,
}

impl<Lang> KindingConclusion<Lang>
where
    Lang: Language,
{
    pub fn new<Ty>(ty: Ty, knd: Kind) -> KindingConclusion<Lang>
    where
        Ty: Into<Lang::Type>,
    {
        KindingConclusion {
            ty: ty.into(),
            kind: knd,
        }
    }
}

impl<Lang> fmt::Display for KindingConclusion<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} :: {}", self.ty, self.kind)
    }
}
