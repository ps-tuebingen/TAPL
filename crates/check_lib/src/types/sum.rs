use crate::Kindcheck;
use common::errors::KindMismatch;
use syntax::{
    kinds::Kind,
    types::{Sum, Type},
};
impl<Ty> Kindcheck<Ty> for Sum<Ty>
where
    Ty: Type + Kindcheck<Ty>,
    <Ty as Kindcheck<Ty>>::CheckError: From<KindMismatch>,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;
    type CheckError = <Ty as Kindcheck<Ty>>::CheckError;

    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Self::CheckError> {
        let left_kind = self.left.check_kind(env)?;
        let right_kind = self.right.check_kind(env)?;
        left_kind.check_equal(&right_kind)?;
        Ok(left_kind)
    }
}
