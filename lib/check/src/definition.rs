use crate::Typecheck;
use derivations::{DefinitionDerivation, Derivation};
use errors::check_error::CheckError;
use syntax::{definition::Definition, env::Environment, language::Language, types::TypeGroup};

impl<Lang> Typecheck for Definition<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
{
    type Lang = Lang;
    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Lang>, CheckError> {
        let body_res = self.body.check(env)?;
        let body_ty = body_res.ret_ty();
        self.annot.check_equal(&body_ty)?;
        Ok(DefinitionDerivation::new(&self.name, body_res.into_ty()?).into())
    }
}
