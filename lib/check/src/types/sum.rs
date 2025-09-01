use crate::Kindcheck;
use errors::check_error::CheckError;
use syntax::{env::Environment, kinds::Kind, language::Language, types::Sum};
impl<Lang> Kindcheck for Sum<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
{
    type Lang = Lang;
    fn check_kind(&self, env: Environment<Self::Lang>) -> Result<Kind, CheckError> {
        let left_kind = self.left.check_kind(env.clone())?;
        let right_kind = self.right.check_kind(env)?;
        left_kind.check_equal(&right_kind)?;
        Ok(left_kind)
    }
}
