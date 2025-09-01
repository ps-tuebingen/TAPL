use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment, language::Language, subst::SubstType, terms::Unfold, types::TypeGroup,
};

impl<Lang> Typecheck for Unfold<Lang>
where
    Lang: Language,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Lang::Term: Typecheck<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let ty_norm = self.ty.clone().normalize(env.clone());
        let ty_kind = ty_norm.check_kind(env.clone())?;

        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty().normalize(env.clone());
        let term_knd = term_ty.check_kind(env.clone())?;
        term_knd.check_equal(&ty_kind)?;

        ty_norm.check_equal(&term_ty)?;
        let mu_ty = term_ty.clone().into_mu()?;
        let ty = mu_ty.ty.subst_type(&mu_ty.var, &term_ty);
        let conc = TypingConclusion::new(env.clone(), self.clone(), ty);
        let deriv = TypingDerivation::unfold(conc, term_res);
        Ok(deriv.into())
    }
}
