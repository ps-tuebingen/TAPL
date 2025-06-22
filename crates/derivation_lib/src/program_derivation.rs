use crate::definition_derivation::DefinitionDerivation;
use syntax::{terms::Term, types::Type};

pub struct ProgramDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub def_derivations: Vec<DefinitionDerivation<T, Ty>>,
    pub main_derivation: Option<DefinitionDerivation<T, Ty>>,
}

impl<T, Ty> ProgramDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new() -> ProgramDerivation<T, Ty> {
        ProgramDerivation {
            def_derivations: vec![],
            main_derivation: None,
        }
    }
}
