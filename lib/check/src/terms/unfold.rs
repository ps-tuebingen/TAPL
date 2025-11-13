use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::{collections::HashSet, rc::Rc};
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
        let features = Lang::features();
        let mut premises = vec![];

        let ty_norm;
        if features.normalizing() {
            let ty_norm_deriv = self.ty.clone().normalize(env.clone());
            ty_norm = ty_norm_deriv.ret_ty();
            premises.push(ty_norm_deriv);
        } else {
            ty_norm = self.ty.clone();
        }

        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty();
        premises.push(term_res);

        let term_ty_norm;
        if features.normalizing() {
            let ty_norm_deriv = term_ty.normalize(env.clone());
            term_ty_norm = ty_norm_deriv.ret_ty();
            premises.push(ty_norm_deriv);
        } else {
            term_ty_norm = term_ty;
        }

        if features.kinded() {
            let ty_res = ty_norm.check_kind(env.clone())?.into_kind()?;
            let term_res = term_ty_norm.check_kind(env.clone())?.into_kind()?;
            term_res.ret_kind().check_equal(&ty_res.ret_kind())?;
            premises.push(ty_res.into());
            premises.push(term_res.into());
        }

        ty_norm.check_equal(&term_ty_norm)?;
        let mu_ty = term_ty_norm.clone().into_mu()?;
        let ty = mu_ty.ty.subst_type(&mu_ty.var, &term_ty_norm);
        let conc = TypingConclusion::new(env, self.clone(), Rc::unwrap_or_clone(ty));
        let deriv = TypingDerivation::unfold(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_unfold()])
    }
}
