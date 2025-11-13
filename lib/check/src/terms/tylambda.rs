use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{env::Environment, language::Language, terms::TyLambda, types::Forall};

impl<Lang> Typecheck for TyLambda<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type: Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Forall<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, mut env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        env.add_tyvar_kind(self.var.clone(), self.annot.clone());

        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty();
        premises.push(term_res);

        let ty_norm;
        if features.normalizing() {
            let ty_norm_deriv = term_ty.normalize(env.clone());
            ty_norm = ty_norm_deriv.ret_ty();
            premises.push(ty_norm_deriv);
        } else {
            ty_norm = term_ty;
        }

        if features.kinded() {
            let term_res = ty_norm.check_kind(env.clone())?.into_kind()?;
            self.annot.check_equal(&term_res.ret_kind())?;
            premises.push(term_res.into());
        }

        let ty = Forall::new(&self.var, self.annot.clone(), ty_norm);
        let conc = TypingConclusion::new(env, self.clone(), ty);
        let deriv = TypingDerivation::tylambda(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        let features = Lang::features();
        HashSet::from([DerivationRule::check_ty_lambda(features.subtyped())])
    }
}
