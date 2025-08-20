use crate::Subtypecheck;
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    types::{Reference, Sink, Source, Top, TypeGroup},
};
impl<Ty> Subtypecheck for Reference<Ty>
where
    Ty: TypeGroup + Subtypecheck<Type = Ty>,
    Top<Ty>: Into<Ty>,
    Reference<Ty>: Into<Ty>,
    Source<Ty>: Into<Ty>,
    Sink<Ty>: Into<Ty>,
{
    type Type = Ty;
    type Term = <Ty as Subtypecheck>::Term;
    fn check_subtype(
        &self,
        sup: &Ty,
        env: Environment<Ty>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into());
        }

        if let Ok(src) = sup.clone().into_source() {
            let src_res = self.ty.check_subtype(&(*src.ty), env.clone())?;
            Ok(SubtypeDerivation::ref_source(env, self.clone(), src.clone(), src_res).into())
        } else if let Ok(sink) = sup.clone().into_sink() {
            let sink_res = sink.ty.check_subtype(&(*sink.ty), env.clone())?;
            Ok(SubtypeDerivation::ref_sink(env, self.clone(), sink.clone(), sink_res).into())
        } else {
            let sup_ref = sup.clone().into_ref()?;
            sup_ref.ty.check_subtype(&(*self.ty), env.clone())?;
            let inner_res = self.ty.check_subtype(&(*sup_ref.ty), env.clone())?;
            Ok(SubtypeDerivation::ref_ref(env, self.clone(), sup_ref.clone(), inner_res).into())
        }
    }
}
