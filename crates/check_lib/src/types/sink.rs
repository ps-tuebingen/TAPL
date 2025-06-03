use crate::Subtypecheck;
use common::errors::TypeMismatch;
use syntax::{
    env::Environment,
    types::{Sink, TypeGroup},
};
impl<Ty> Subtypecheck<Ty> for Sink<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
    <Ty as Subtypecheck<Ty>>::CheckError: From<TypeMismatch>,
{
    type CheckError = <Ty as Subtypecheck<Ty>>::CheckError;

    fn check_subtype(&self, sup: &Ty, env: &mut Environment<Ty>) -> Result<(), Self::CheckError> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_sink = sup.clone().into_sink()?;
        sup_sink.ty.check_subtype(&(*self.ty), env)
    }
}
