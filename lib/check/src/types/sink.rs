use crate::Subtypecheck;
use derivations::{Derivation, SubtypeDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    types::{Sink, Top, TypeGroup},
};
impl<Ty> Subtypecheck for Sink<Ty>
where
    Ty: TypeGroup + Subtypecheck<Type = Ty>,
    Top<Ty>: Into<Ty>,
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

        let sup_sink = sup.clone().into_sink()?;
        let sup_res = sup_sink.ty.check_subtype(&(*self.ty), env.clone())?;
        Ok(SubtypeDerivation::sink(env, self.clone(), sup_sink, sup_res).into())
    }
}
