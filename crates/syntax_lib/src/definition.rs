use crate::{terms::Term, types::Type, Name};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Definition<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub name: Name,
    pub annot: Ty,
    pub body: T,
}

impl<T, Ty> Definition<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new<T1, Ty1>(name: &str, annot: Ty1, body: T1) -> Definition<T, Ty>
    where
        T1: Into<T>,
        Ty1: Into<Ty>,
    {
        Definition {
            name: name.to_owned(),
            annot: annot.into(),
            body: body.into(),
        }
    }
}

impl<T, Ty> fmt::Display for Definition<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}::{}:={}", self.name, self.annot, self.body)
    }
}
