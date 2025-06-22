use crate::definition_derivation::DefinitionDerivation;
use std::collections::HashMap;
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

    pub fn tys(&self) -> HashMap<String, Ty> {
        let mut tys = HashMap::new();
        for df_deriv in self.def_derivations.iter() {
            tys.insert(df_deriv.name.clone(), df_deriv.ty());
        }

        if let Some(ref md) = self.main_derivation {
            tys.insert(md.name.clone(), md.ty());
        }
        tys
    }
}
