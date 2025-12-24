use crate::Kindcheck;
use derivations::{Derivation, KindingDerivation};
use errors::{KindMismatch, check_error::CheckError};
use grammar::{DerivationRule, Symbol, symbols::SpecialChar};
use std::collections::HashSet;
use syntax::{env::Environment, language::Language, types::Sum};

impl<Lang> Kindcheck for Sum<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn check_kind(&self, env: Environment<Self::Lang>) -> Result<Derivation<Lang>, CheckError> {
        let left_res = self.left.check_kind(env.clone())?.into_kind()?;
        let right_res = self.right.check_kind(env)?.into_kind()?;
        let right_kind = right_res.ret_kind();
        let left_kind = left_res.ret_kind();
        if left_kind != right_kind {
            return Err(KindMismatch::new(
                left_kind.to_string(),
                right_kind.to_string(),
                self.span,
            )
            .into());
        }
        Ok(KindingDerivation::sum(self.clone(), right_kind, left_res, right_res).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::norm_cong(|sym| {
                Symbol::paren(vec![sym, SpecialChar::Plus.into(), Symbol::Type])
            }),
            DerivationRule::norm_cong(|sym| {
                Symbol::paren(vec![Symbol::Type, SpecialChar::Plus.into(), sym])
            }),
        ])
    }
}
