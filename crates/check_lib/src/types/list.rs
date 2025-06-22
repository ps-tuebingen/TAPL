use crate::{errors::CheckError, Subtypecheck};
use syntax::{
    env::Environment,
    types::{List, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for List<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    fn check_subtype(&self, sup: &Ty, env: Environment<Ty>) -> Result<(), CheckError> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_list = sup.clone().into_list()?;
        self.ty.check_subtype(&(*sup_list.ty), env)
    }
}
