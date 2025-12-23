use crate::Subtypecheck;
use derivations::{Derivation, SubtypeDerivation};
use errors::{TypeMismatch, check_error::CheckError};
use grammar::{
    DerivationRule,
    symbols::{Keyword, Symbol},
};
use std::collections::HashSet;
use syntax::{
    env::Environment,
    language::Language,
    types::{Reference, Sink, Source, Top, TypeGroup},
};

impl<Lang> Subtypecheck for Reference<Lang>
where
    Lang: Language,
    Top<Lang>: Into<Lang::Type>,
    Self: Into<Lang::Type>,
    Source<Lang>: Into<Lang::Type>,
    Sink<Lang>: Into<Lang::Type>,
    Lang::Type: Subtypecheck<Lang = Lang> + TypeGroup<Lang = Lang>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        if let Some(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(
                env,
                self.clone(),
                top.kind,
                self.span,
                Vec::new(),
            )
            .into());
        }

        if let Some(src) = sup.clone().into_source() {
            let src_res = self.ty.check_subtype(&(*src.ty), env.clone())?;
            Ok(SubtypeDerivation::ref_source(env, self.clone(), src, src_res).into())
        } else if let Some(sink) = sup.clone().into_sink() {
            let sink_res = sink.ty.check_subtype(&(*self.ty), env.clone())?;
            Ok(SubtypeDerivation::ref_sink(env, self.clone(), sink, sink_res).into())
        } else {
            let sup_ref = sup.clone().into_ref().ok_or_else(|| {
                TypeMismatch::new(sup.to_string(), "Reference Type".to_string(), self.span)
            })?;
            sup_ref.ty.check_subtype(&(*self.ty), env.clone())?;
            let inner_res = self.ty.check_subtype(&(*sup_ref.ty), env.clone())?;
            Ok(SubtypeDerivation::ref_ref(env, self.clone(), sup_ref, inner_res).into())
        }
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::sub_cong(|sym| vec![Keyword::Ref.into(), Symbol::sqbrack(sym)].into()),
            DerivationRule::sub_ref_sink(),
            DerivationRule::sub_ref_source(),
        ])
    }
}
