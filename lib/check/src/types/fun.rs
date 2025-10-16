use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
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
            return Ok(SubtypeDerivation::sub_top(env, self.clone(), top.kind).into());
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
{
    type Lang = Lang;
    fn check_kind(&self, env: Environment<Self::Lang>) -> Result<Kind, CheckError> {
        let from_kind = self.from.check_kind(env.clone())?;
        if from_kind != Kind::Star {
            return Err(KindMismatch::new(from_kind.to_string(), Kind::Star.to_string()).into());
        };

        let to_kind = self.to.check_kind(env)?;
        if to_kind != Kind::Star {
            return Err(KindMismatch::new(to_kind.to_string(), Kind::Star.to_string()).into());
        }
        Ok(Kind::Star)
    }
}

impl<Lang> Normalize for Fun<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang>,
{
    type Lang = Lang;
    fn normalize(self, env: Environment<Self::Lang>) -> <Self::Lang as Language>::Type {
        let from_norm = self.from.normalize(env.clone());
        let to_norm = self.to.normalize(env);
        Fun {
            from: Rc::new(from_norm),
            to: Rc::new(to_norm),
        }
        .into()
    }
}
