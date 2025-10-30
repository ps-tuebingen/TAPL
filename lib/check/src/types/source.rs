use crate::Subtypecheck;
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, Symbol},
};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    types::{Source, Top, TypeGroup},
};

impl<Lang> Subtypecheck for Source<Lang>
where
    Lang: Language,
    Top<Lang>: Into<Lang::Type>,
    Source<Lang>: Into<Lang::Type>,
    Lang::Type: Subtypecheck<Lang = Lang>,
    Lang::Type: TypeGroup<Lang = Lang>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind, vec![]).into());
        }

        let sup_src = sup.clone().into_source()?;
        let inner_res = self.ty.check_subtype(&(*sup_src.ty), env.clone())?;
        Ok(SubtypeDerivation::source(env, self.clone(), sup_src, inner_res).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::sub_cong(|sym| {
            vec![Keyword::Source.into(), Symbol::sqbrack(sym)].into()
        })])
    }
}
