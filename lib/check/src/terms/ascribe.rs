use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, terms::Ascribe, types::TypeGroup};

impl<Lang> Typecheck for Ascribe<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let t_res = self.term.check(env.clone())?;
        let t_ty = t_res.ret_ty().normalize(env.clone());
        let asc_norm = self.ty.clone().normalize(env.clone());
        let t_kind = t_ty.check_kind(env.clone())?;
        let ty_kind = self.ty.check_kind(env.clone())?;
        t_kind.check_equal(&ty_kind)?;
        asc_norm.check_equal(&t_ty)?;

        let conc = TypingConclusion::new(env, self.clone(), self.ty.clone());
        let deriv = TypingDerivation::ascribe(conc, t_res);
        Ok(deriv.into())
    }
}
