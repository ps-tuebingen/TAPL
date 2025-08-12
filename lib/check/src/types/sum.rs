use crate::Kindcheck;
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Sum, Type},
};
impl<Ty> Kindcheck<Ty> for Sum<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, env: Environment<Ty>) -> Result<Kind, CheckError> {
        let left_kind = self.left.check_kind(env.clone())?;
        let right_kind = self.right.check_kind(env)?;
        left_kind.check_equal(&right_kind)?;
        Ok(left_kind)
    }
}
