use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::{KindMismatch, TypeMismatch, check_error::CheckError};
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    terms::IsZero,
    types::{Bool, TypeGroup},
};

impl<Lang> Typecheck for IsZero<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type:
        TypeGroup<Lang = Lang> + Normalize<Lang = Lang> + Kindcheck<Lang = Lang>,
    Bool<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn check(&self, env: Environment<Lang>) -> Result<Derivation<Self::Lang>, CheckError> {
        let features = Lang::features();
        let mut premises = vec![];

        let inner_res = self.term.check(env.clone())?;
        let inner_ty = inner_res.ret_ty();
        premises.push(inner_res);

        let inner_norm;
        if features.normalizing() {
            let inner_norm_res = inner_ty.normalize(env.clone());
            inner_norm = inner_norm_res.ret_ty();
            premises.push(inner_norm_res);
        } else {
            inner_norm = inner_ty;
        }

        if features.kinded() {
            let inner_res = inner_norm.check_kind(env.clone())?.into_kind()?;
            let inner_knd = inner_res.ret_kind();
            inner_knd.clone().into_star().ok_or_else(|| {
                KindMismatch::new(inner_knd.to_string(), "Star Kind".to_string(), self.span)
            })?;
            premises.push(inner_res.into());
        }

        inner_norm.clone().into_nat().ok_or_else(|| {
            TypeMismatch::new(inner_norm.to_string(), "Nat".to_string(), self.span)
        })?;

        let conc = TypingConclusion::new(env, self.clone(), Bool::new(self.span));
        let deriv = TypingDerivation::iszero(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_cong(
            vec![
                Keyword::IsZero.into(),
                Symbol::paren(Symbol::Term),
                SpecialChar::ParenC.into(),
            ],
            Keyword::Bool,
            Keyword::Nat,
            "T-IsZero",
        )])
    }
}
