use syntax::{env::Environment, terms::Term, types::Type};

pub struct Conclusion<T, Ty>
where
    T: Term,
    Ty: Type,
{
    env: Environment<Ty>,
    term: T,
    pub(crate) ty: Ty,
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
