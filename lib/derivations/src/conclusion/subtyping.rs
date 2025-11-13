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
    pub fn new<Ty1, Ty2>(env: Environment<Lang>, sub_ty: Ty1, super_ty: Ty2) -> Self
    where
        Ty1: Into<Lang::Type>,
        Ty2: Into<Lang::Type>,
    {
        Self {
            env,
            sub: sub_ty.into(),
            sup: super_ty.into(),
        }
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
