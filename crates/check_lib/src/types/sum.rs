use crate::{Kindcheck, Subtypecheck};
use common::errors::Error;
use syntax::types::{Sum, TypeGroup};

impl<Ty> Kindcheck<Ty> for Sum<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;
    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        let left_kind = self.left.check_kind(env)?;
        let right_kind = self.right.check_kind(env)?;
        left_kind.check_equal(&right_kind).map_err(to_kind_err)?;
        Ok(left_kind)
    }
}
