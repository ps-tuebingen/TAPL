use super::TypingDerivation;
use std::fmt;
use syntax::{terms::Term, types::Type};

#[derive(Debug)]
pub struct DefinitionDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub name: String,
    pub body_derivation: TypingDerivation<T, Ty>,
}

impl<T, Ty> DefinitionDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new(
        name: &str,
        body_derivation: TypingDerivation<T, Ty>,
    ) -> DefinitionDerivation<T, Ty> {
        DefinitionDerivation {
            name: name.to_owned(),
            body_derivation,
        }
    }

    pub fn ret_ty(&self) -> Ty {
        self.body_derivation.ret_ty()
    }
}

impl<T, Ty> fmt::Display for DefinitionDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.body_derivation)
    }
}
