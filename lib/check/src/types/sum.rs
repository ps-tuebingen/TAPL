use crate::Kindcheck;
use derivations::{Derivation, KindingDerivation};
use errors::check_error::CheckError;
use syntax::{env::Environment, language::Language, types::Sum};

impl<Lang> Kindcheck for Sum<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn check_kind(&self, env: Environment<Self::Lang>) -> Result<Derivation<Lang>, CheckError> {
        let left_res = self.left.check_kind(env.clone())?.into_kind()?;
        let right_res = self.right.check_kind(env)?.into_kind()?;
        let right_kind = right_res.ret_kind();
        left_res.ret_kind().check_equal(&right_kind)?;
        Ok(KindingDerivation::sum(self.clone(), right_kind, left_res, right_res).into())
    }
}
