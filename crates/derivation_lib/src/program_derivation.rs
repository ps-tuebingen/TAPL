use crate::{definition_derivation::DefinitionDerivation, typing_derivation::TypingDerivation};
use std::collections::HashMap;
use syntax::{terms::Term, types::Type};

#[derive(Debug)]
pub struct ProgramDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub def_derivations: Vec<DefinitionDerivation<T, Ty>>,
    pub main_derivation: TypingDerivation<T, Ty>,
}

impl<T, Ty> ProgramDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    pub fn new(
        main_derivation: TypingDerivation<T, Ty>,
        def_derivations: Vec<DefinitionDerivation<T, Ty>>,
    ) -> ProgramDerivation<T, Ty> {
        ProgramDerivation {
            def_derivations,
            main_derivation,
        }
    }

    pub fn tys(&self) -> HashMap<String, Ty> {
        let mut tys = HashMap::new();
        for df_deriv in self.def_derivations.iter() {
            tys.insert(df_deriv.name.clone(), df_deriv.ty());
        }

        tys.insert("main".to_owned(), self.main_derivation.ty());
        tys
    }
}
