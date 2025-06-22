use crate::{Subtypecheck, errors::CheckError};
use syntax::{
    env::Environment,
    types::{Source, TypeGroup},
};
impl<Ty> Subtypecheck<Ty> for Source<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    fn check_subtype(&self, sup: &Ty, env: Environment<Ty>) -> Result<(), CheckError> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_src = sup.clone().into_source()?;
        self.ty.check_subtype(&(*sup_src.ty), env)
    }
}
