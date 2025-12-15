use crate::Typecheck;
use derivations::{DefinitionDerivation, Derivation};
use errors::{TypeMismatch, check_error::CheckError};
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{definition::Definition, env::Environment, language::Language};

impl<Lang> Typecheck for Definition<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
{
    type Lang = Lang;
    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Lang>, CheckError> {
        let body_res = self.body.check(env)?;
        let body_ty = body_res.ret_ty();
        if self.annot != body_ty {
            return Err(TypeMismatch::new(self.annot.to_string(), body_ty.to_string()).into());
        }
        Ok(DefinitionDerivation::new(&self.name, body_res.into_ty()?).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        <Lang::Term as Typecheck>::rules()
    }
}
