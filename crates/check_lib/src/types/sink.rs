use crate::{errors::CheckError, Subtypecheck};
use syntax::{
    env::Environment,
    types::{Sink, TypeGroup},
};
impl<Ty> Subtypecheck<Ty> for Sink<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    fn check_subtype(&self, sup: &Ty, env: Environment<Ty>) -> Result<(), CheckError<Ty>> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_sink = sup.clone().into_sink()?;
        sup_sink.ty.check_subtype(&(*self.ty), env)
    }
}
