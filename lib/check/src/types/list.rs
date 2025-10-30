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
    types::{List, Top, TypeGroup},
};

impl<Lang> Subtypecheck for List<Lang>
where
    Lang: Language,
    Top<Lang>: Into<Lang::Type>,
    List<Lang>: Into<Lang::Type>,
    Lang::Type: Subtypecheck<Lang = Lang> + TypeGroup<Lang = Lang>,
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

        let sup_list = sup.clone().into_list()?;
        let sup_res = self.ty.check_subtype(&(*sup_list.ty), env.clone())?;
        Ok(SubtypeDerivation::list(env, self.clone(), sup.clone(), sup_res).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::sub_cong(|sym| {
            vec![
                Keyword::List.into(),
                Symbol::sqbrack(Symbol::Type),
                Symbol::paren(vec![sym]),
            ]
            .into()
        })])
    }
}
