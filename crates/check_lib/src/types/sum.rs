use crate::Kindcheck;
use common::errors::KindMismatch;
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Sum, Type},
};
impl<Ty> Kindcheck<Ty> for Sum<Ty>
where
    Ty: Type + Kindcheck<Ty>,
    <Ty as Kindcheck<Ty>>::CheckError: From<KindMismatch>,
{
    type CheckError = <Ty as Kindcheck<Ty>>::CheckError;

    fn check_kind(&self, env: Environment<Ty>) -> Result<Kind, Self::CheckError> {
        let left_kind = self.left.check_kind(env.clone())?;
        let right_kind = self.right.check_kind(env)?;
        left_kind.check_equal(&right_kind)?;
        Ok(left_kind)
    }
}
