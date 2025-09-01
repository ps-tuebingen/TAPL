use crate::Subtypecheck;
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
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
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into());
        }

        let sup_sink = sup.clone().into_sink()?;
        let sup_res = sup_sink.ty.check_subtype(&(*self.ty), env.clone())?;
        Ok(SubtypeDerivation::sink(env, self.clone(), sup_sink, sup_res).into())
    }
}
