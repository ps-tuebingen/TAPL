use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, KindingDerivation, NormalizingDerivation, SubtypeDerivation};
use errors::KindMismatch;
use errors::check_error::CheckError;
use std::rc::Rc;
use syntax::{
    env::Environment,
    kinds::Kind,
    language::Language,
    types::{Fun, Top, TypeGroup},
};

impl<Lang> Subtypecheck for Fun<Lang>
where
    Lang: Language,
    Fun<Lang>: Into<Lang::Type>,
    Top<Lang>: Into<Lang::Type>,
    Lang::Type: Subtypecheck<Lang = Lang> + TypeGroup<Lang = Lang>,
{
    type Lang = Lang;
    fn check_subtype(
        &self,
        sup: &<Lang as Language>::Type,
        env: Environment<Self::Lang>,
    ) -> Result<Derivation<Self::Lang>, CheckError> {
        if let Ok(top) = sup.clone().into_top() {
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind, vec![]).into());
        }

        let sup_fun = sup.clone().into_fun()?;
        let from_res = sup_fun.from.check_subtype(&(*self.from), env.clone())?;
        let to_res = self.to.check_subtype(&(*sup_fun.to), env.clone())?;
        Ok(SubtypeDerivation::fun(env, self.clone(), sup.clone(), from_res, to_res).into())
    }
}

impl<Lang> Kindcheck for Fun<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
    Self: Into<Lang::Type>,
{
    type Lang = Lang;
    fn check_kind(&self, env: Environment<Self::Lang>) -> Result<Derivation<Lang>, CheckError> {
        let from_res = self.from.check_kind(env.clone())?.into_kind()?;
        let from_kind = from_res.ret_kind();
        if from_kind != Kind::Star {
            return Err(KindMismatch::new(from_kind.to_string(), Kind::Star.to_string()).into());
        };

        let to_res = self.to.check_kind(env)?.into_kind()?;
        let to_kind = to_res.ret_kind();
        if to_kind != Kind::Star {
            return Err(KindMismatch::new(to_kind.to_string(), Kind::Star.to_string()).into());
        }
        Ok(KindingDerivation::fun(self.clone(), from_res, to_res).into())
    }
}

impl<Lang> Normalize for Fun<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang>,
{
    type Lang = Lang;
    fn normalize(self, env: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        let from_norm = self.from.clone().normalize(env.clone());
        let to_norm = self.to.clone().normalize(env);
        let self_norm = Fun {
            from: Rc::new(from_norm.ret_ty()),
            to: Rc::new(to_norm.ret_ty()),
        };
        NormalizingDerivation::cong(self, self_norm, vec![from_norm, to_norm]).into()
    }
}
