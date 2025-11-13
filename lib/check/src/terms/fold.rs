use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::collections::HashSet;
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
        let features = Lang::features();
        let mut premises = vec![];

        let ty_norm;
        if features.normalizing() {
            let norm_deriv = self.ty.clone().normalize(env.clone());
            ty_norm = norm_deriv.ret_ty();
            premises.push(norm_deriv);
        } else {
            ty_norm = self.ty.clone();
        }

        let mu_ty = ty_norm.into_mu()?;
        env.add_tyvar_kind(mu_ty.var.clone(), Kind::Star);
        if features.kinded() {
            let mu_res = mu_ty.ty.check_kind(env.clone())?.into_kind()?;
            mu_res.ret_kind().into_star()?;
            premises.push(mu_res.into());
        }

        let mu_subst = mu_ty
            .ty
            .clone()
            .subst_type(&mu_ty.var.clone(), &(mu_ty.into()));
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty();
        premises.push(term_res);

        let term_norm;
        if features.normalizing() {
            let term_norm_deriv = term_ty.normalize(env.clone());
            term_norm = term_norm_deriv.ret_ty();
            premises.push(term_norm_deriv);
        } else {
            term_norm = term_ty;
        }

        if features.kinded() {
            let term_res = term_norm.check_kind(env.clone())?.into_kind()?;
            term_res.ret_kind().into_star()?;
            premises.push(term_res.into());
        }
        term_norm.check_equal(&mu_subst)?;

        let conc = TypingConclusion::new(env, self.clone(), self.ty.clone());
        let deriv = TypingDerivation::fold(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        let mu_ty = vec![
            SpecialChar::Mu.into(),
            Symbol::Typevariable,
            SpecialChar::Dot.into(),
            Symbol::Type,
        ];
        HashSet::from([DerivationRule::check_cong(
            vec![
                Keyword::Fold.into(),
                Symbol::sqbrack(mu_ty.clone()),
                Symbol::Term,
            ],
            mu_ty.clone(),
            vec![
                Symbol::Type,
                Symbol::sqbrack(Symbol::mapto(Symbol::Typevariable, mu_ty)),
            ],
            "T-Fold",
        )])
    }
}
