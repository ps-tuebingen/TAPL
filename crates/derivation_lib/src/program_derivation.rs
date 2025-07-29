use crate::{definition_derivation::DefinitionDerivation, typing_derivation::TypingDerivation};
use std::{collections::HashMap, fmt};
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

impl<T, Ty> fmt::Display for ProgramDerivation<T, Ty>
where
    T: Term,
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for def_deriv in self.def_derivations.iter() {
            writeln!(f, "{def_deriv}")?;
        }
        write!(f, "{}", self.main_derivation)
    }
}
