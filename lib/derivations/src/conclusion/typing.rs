use super::Conclusion;
use std::fmt;
use syntax::{env::Environment, terms::Term, types::Type};

#[derive(Debug)]
pub struct TypingConclusion<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub env: Environment<Ty>,
    pub term: T,
    pub ty: Ty,
}

impl<T, Ty> TypingConclusion<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<T1, Ty1>(env: Environment<Ty>, term: T1, ty: Ty1) -> TypingConclusion<T, Ty>
    where
        T1: Into<T>,
        Ty1: Into<Ty>,
    {
        TypingConclusion {
            env,
            term: term.into(),
            ty: ty.into(),
        }
    }

    pub fn ty(&self) -> Ty {
        self.ty.clone()
    }
}

impl<T, Ty> From<TypingConclusion<T, Ty>> for Conclusion<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn from(conc: TypingConclusion<T, Ty>) -> Conclusion<T, Ty> {
        Conclusion::Typing(conc)
    }
}

impl<T, Ty> fmt::Display for TypingConclusion<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} |-> {} : {}", self.env, self.term, self.ty)
    }
}
