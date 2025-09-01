use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Cast, types::TypeGroup};

impl<Lang> Typecheck for Cast<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty().normalize(env.clone());
        let ty_norm = self.ty.clone().normalize(env.clone());
        let term_kind = term_ty.check_kind(env.clone())?;
        let ty_kind = ty_norm.check_kind(env.clone())?;
        term_kind.check_equal(&ty_kind)?;

        let conc = TypingConclusion::new(env, self.clone(), ty_norm.clone());
        let deriv = TypingDerivation::cast(conc, term_res);
        Ok(deriv.into())
    }
}
