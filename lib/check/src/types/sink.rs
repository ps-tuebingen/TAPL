use crate::Subtypecheck;
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    types::{Sink, Top, TypeGroup},
};

impl<Lang> Subtypecheck for Sink<Lang>
where
    Lang: Language,
    Top<Lang>: Into<Lang::Type>,
    Sink<Lang>: Into<Lang::Type>,
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

        let sup_sink = sup.clone().into_sink()?;
        let sup_res = sup_sink.ty.check_subtype(&(*self.ty), env.clone())?;
        Ok(SubtypeDerivation::sink(env, self.clone(), sup_sink, sup_res).into())
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::sub_cong(|sym| Symbol::Prefixed {
            prefix: Box::new(Keyword::Sink.into()),
            inner: Box::new(Symbol::Delim {
                delim_open: SpecialChar::SqBrackO,
                inner: Box::new(sym),
                delim_close: SpecialChar::SqBrackC,
            }),
        })])
    }
}
