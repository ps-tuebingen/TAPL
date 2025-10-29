use crate::Typecheck;
use derivations::{Derivation, ProgramDerivation};
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{env::Environment, language::Language, program::Program};

impl<Lang> Typecheck for Program<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
{
    type Lang = Lang;

    fn check(&self, mut env: Environment<Lang>) -> Result<Derivation<Lang>, CheckError> {
        let mut derivs = vec![];
        for def in self.definitions.iter() {
            env.add_definition(def.name.clone(), def.annot.clone());
        }
        for def in self.definitions.iter() {
            let def_res = def.check(env.clone())?;
            derivs.push(def_res.into_def()?);
        }

        let main_res = self.main.check(env)?;
        Ok(ProgramDerivation::new(main_res.into_ty()?, derivs).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        <Lang::Term as Typecheck>::rules()
    }
}
