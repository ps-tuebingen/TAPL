use super::TypingDerivation;
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

    pub fn ty(&self) -> Ty {
        self.body_derivation.ty()
    }
}
