use super::Conclusion;
use std::fmt;
use syntax::{env::Environment, language::Language};

#[derive(Debug)]
pub struct SubtypeConclusion<Lang>
where
    Lang: Language,
{
    pub env: Environment<Lang>,
    pub sub: Lang::Type,
    pub sup: Lang::Type,
}

impl<Lang> SubtypeConclusion<Lang>
where
    Lang: Language,
{
    pub fn new<Ty1, Ty2>(env: Environment<Lang>, sub: Ty1, sup: Ty2) -> SubtypeConclusion<Lang>
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        SubtypeConclusion {
            env,
            sub: sub.into(),
            sup: sup.into(),
        }
    }
}

impl<Lang> From<SubtypeConclusion<Lang>> for Conclusion<Lang>
where
    Lang: Language,
{
    fn from(conc: SubtypeConclusion<Lang>) -> Conclusion<Lang> {
        Conclusion::Subtyping(conc)
    }
}

impl<Lang> fmt::Display for SubtypeConclusion<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} |-> {} <: {}", self.env, self.sub, self.sup)
    }
}
