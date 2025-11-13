use crate::{DefinitionDerivation, TypingDerivation};
use std::{collections::HashMap, fmt};
use syntax::language::Language;

#[derive(Debug)]
pub struct ProgramDerivation<Lang>
where
    Lang: Language,
{
    pub def_derivations: Vec<DefinitionDerivation<Lang>>,
    pub main_derivation: TypingDerivation<Lang>,
}

impl<Lang> ProgramDerivation<Lang>
where
    Lang: Language,
{
    pub const fn new(
        main_derivation: TypingDerivation<Lang>,
        def_derivations: Vec<DefinitionDerivation<Lang>>,
    ) -> Self {
        Self {
            def_derivations,
            main_derivation,
        }
    }

    pub fn ret_ty(&self) -> Lang::Type {
        self.main_derivation.ret_ty()
    }

    pub fn tys(&self) -> HashMap<String, Lang::Type> {
        let mut tys = HashMap::new();
        for df_deriv in &self.def_derivations {
            tys.insert(df_deriv.name.clone(), df_deriv.ret_ty());
        }

        tys.insert("main".to_owned(), self.main_derivation.ret_ty());
        tys
    }
}

impl<Lang> fmt::Display for ProgramDerivation<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for def_deriv in &self.def_derivations {
            writeln!(f, "{def_deriv}")?;
        }
        write!(f, "{}", self.main_derivation)
    }
}
