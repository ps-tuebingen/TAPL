use crate::Subtypecheck;
use common::errors::TypeMismatch;
use syntax::{
    env::Environment,
    types::{Source, TypeGroup},
};
impl<Ty> Subtypecheck<Ty> for Source<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
    <Ty as Subtypecheck<Ty>>::CheckError: From<TypeMismatch>,
{
    type CheckError = <Ty as Subtypecheck<Ty>>::CheckError;

    fn check_subtype(&self, sup: &Ty, env: Environment<Ty>) -> Result<(), Self::CheckError> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_src = sup.clone().into_source()?;
        self.ty.check_subtype(&(*sup_src.ty), env)
    }
}
