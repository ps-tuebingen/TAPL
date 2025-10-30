use crate::{Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, Symbol},
};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    terms::Pred,
    types::{Nat, TypeGroup},
};

impl<Lang> Typecheck for Pred<Lang>
where
    Lang: Language,
    Lang::Term: Typecheck<Lang = Lang>,
    <Lang as Language>::Type: TypeGroup<Lang = Lang> + Normalize<Lang = Lang>,
    Nat<Lang>: Into<Lang::Type>,
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
        if features.normalizing {
            let inner_norm_deriv = inner_ty.normalize(env.clone());
            inner_norm = inner_norm_deriv.ret_ty();
            premises.push(inner_norm_deriv);
        } else {
            inner_norm = inner_ty;
        }

        let nat = inner_norm.into_nat()?;
        let conc = TypingConclusion::new(env, self.clone(), nat);
        let deriv = TypingDerivation::pred(conc, premises);
        Ok(deriv.into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::check_cong(
            vec![Keyword::Pred.into(), Symbol::paren(vec![Symbol::Term])],
            Keyword::Nat,
            Keyword::Nat,
            "T-Pred",
        )])
    }
}
