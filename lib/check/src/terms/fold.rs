use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    kinds::Kind,
    language::Language,
    subst::SubstType,
    terms::Fold,
    types::{Mu, TypeGroup},
};

impl<Lang> Typecheck for Fold<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Mu<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, mut env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let mu_ty = self.ty.clone().normalize(env.clone()).into_mu()?;
        env.add_tyvar_kind(mu_ty.var.clone(), Kind::Star);
        mu_ty.ty.check_kind(env.clone())?.into_star()?;

        let mu_subst = mu_ty
            .ty
            .clone()
            .subst_type(&mu_ty.var.clone(), &(mu_ty.into()));
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty().normalize(env.clone());
        term_ty.check_kind(env.clone())?.into_star()?;
        term_ty.check_equal(&mu_subst)?;

        let conc = TypingConclusion::new(env, self.clone(), self.ty.clone());
        let deriv = TypingDerivation::fold(conc, term_res);
        Ok(deriv.into())
    }
}
