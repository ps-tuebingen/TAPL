use super::Conclusion;
use std::fmt;
use syntax::{env::Environment, terms::Term, types::Type};
#[derive(Debug)]
pub struct SubtypeConclusion<Ty>
where
    Ty: Type,
{
    pub env: Environment<Ty>,
    pub sub: Ty,
    pub sup: Ty,
}

impl<Ty> SubtypeConclusion<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1, Ty2>(env: Environment<Ty>, sub: Ty1, sup: Ty2) -> SubtypeConclusion<Ty>
    where
        Ty1: Into<Ty>,
        Ty2: Into<Ty>,
    {
        SubtypeConclusion {
            env,
            sub: sub.into(),
            sup: sup.into(),
        }
    }
}

impl<T, Ty> From<SubtypeConclusion<Ty>> for Conclusion<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(conc: SubtypeConclusion<Ty>) -> Conclusion<T, Ty> {
        Conclusion::Subtyping(conc)
    }
}

impl<Ty> fmt::Display for SubtypeConclusion<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} |-> {} <: {}", self.env, self.sub, self.sup)
    }
}
