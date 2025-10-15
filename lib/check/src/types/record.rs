use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::UndefinedLabel;
use errors::check_error::CheckError;
use std::collections::HashMap;
use syntax::{
    env::Environment,
    kinds::Kind,
    language::Language,
    types::{Record, Top, TypeGroup},
};
impl<Lang> Subtypecheck for Record<Lang>
where
    Lang: Language,
    Top<Lang>: Into<Lang::Type>,
    Record<Lang>: Into<Lang::Type>,
    Lang::Type: Subtypecheck<Lang = Lang> + Normalize<Lang = Lang> + TypeGroup<Lang = Lang>,
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

        let sup_norm = sup.clone().normalize(env.clone());
        let sup_rec = sup_norm.into_record()?;
        let mut inner_res = vec![];
        for (lb, ty) in sup_rec.records.iter() {
            let sub_ty = self.records.get(lb).ok_or(UndefinedLabel::new(lb))?;
            inner_res.push(sub_ty.check_subtype(ty, env.clone())?);
        }
        Ok(SubtypeDerivation::record(env, self.clone(), sup.clone(), inner_res).into())
    }
}

impl<Lang> Kindcheck for Record<Lang>
where
    Lang: Language,
    Lang::Type: Kindcheck<Lang = Lang>,
{
    type Lang = Lang;
    fn check_kind(&self, env: Environment<Self::Lang>) -> Result<Kind, CheckError> {
        for (_, t) in self.records.iter() {
            t.check_kind(env.clone())?.into_star()?;
        }
        Ok(Kind::Star)
    }
}

impl<Lang> Normalize for Record<Lang>
where
    Lang: Language,
    Self: Into<Lang::Type>,
    Lang::Type: Normalize<Lang = Lang>,
{
    type Lang = Lang;
    fn normalize(self, env: Environment<Self::Lang>) -> <Self::Lang as Language>::Type {
        let mut recs_norm = HashMap::new();
        for (lb, ty) in self.records {
            let ty_norm = ty.normalize(env.clone());
            recs_norm.insert(lb, ty_norm);
        }
        Record { records: recs_norm }.into()
    }
}
