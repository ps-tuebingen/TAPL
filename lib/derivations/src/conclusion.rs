use std::fmt;
use syntax::{env::Environment, terms::Term, types::Type};

#[derive(Debug)]
pub struct Conclusion<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub env: Environment<Ty>,
    pub term: T,
    pub ty: Ty,
}

impl<T, Ty> Conclusion<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<T1, Ty1>(env: Environment<Ty>, term: T1, ty: Ty1) -> Conclusion<T, Ty>
    where
        T1: Into<T>,
        Ty1: Into<Ty>,
    {
        Conclusion {
            env,
            term: term.into(),
            ty: ty.into(),
        }
    }
}

impl<T, Ty> fmt::Display for Conclusion<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} |-> {} : {}", self.env, self.term, self.ty)
    }
}
