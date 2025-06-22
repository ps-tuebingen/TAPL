use crate::{errors::CheckError, Kindcheck, Normalize, Subtypecheck};
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Bool, Type, TypeGroup},
};

impl<Ty> Subtypecheck<Ty> for Bool<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty>,
{
    fn check_subtype(&self, sup: &Ty, _: Environment<Ty>) -> Result<(), CheckError> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        sup.clone().into_bool()?;
        Ok(())
    }
}

impl<Ty> Kindcheck<Ty> for Bool<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, _: Environment<Ty>) -> Result<Kind, CheckError> {
        Ok(Kind::Star)
    }
}

impl<Ty> Normalize<Ty> for Bool<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, _: Environment<Ty>) -> Ty {
        self.into()
    }
}
