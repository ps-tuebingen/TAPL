use std::fmt;
use syntax::language::Language;

#[derive(Debug)]
pub struct NormalizingConclusion<Lang>
where
    Lang: Language,
{
    pub left: Lang::Type,
    pub right: Lang::Type,
}

impl<Lang> NormalizingConclusion<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2>(left: Ty1, right: Ty2) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            left: left.into(),
            right: right.into(),
        }
    }
}

impl<Lang> fmt::Display for NormalizingConclusion<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} == {}", self.left, self.right)
    }
}
