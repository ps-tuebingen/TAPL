use crate::Subtypecheck;
use common::errors::TypeMismatch;
use syntax::{
    env::Environment,
    types::{Reference, TypeGroup},
};
impl<Ty> Subtypecheck<Ty> for Reference<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
    <Ty as Subtypecheck<Ty>>::CheckError: From<TypeMismatch>,
{
    type CheckError = <Ty as Subtypecheck<Ty>>::CheckError;

    fn check_subtype(&self, sup: &Ty, env: Environment<Ty>) -> Result<(), Self::CheckError> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        if let Ok(src) = sup.clone().into_source() {
            self.ty.check_subtype(&(*src.ty), env)
        } else if let Ok(sink) = sup.clone().into_sink() {
            sink.ty.check_subtype(&(*sink.ty), env)
        } else {
            let sup_ref = sup.clone().into_ref()?;
            sup_ref.ty.check_subtype(&(*self.ty), env.clone())?;
            self.ty.check_subtype(&(*sup_ref.ty), env)
        }
    }
}
