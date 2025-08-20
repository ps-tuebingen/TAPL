use crate::{Kindcheck, Normalize, Subtypecheck};
use derivations::{Derivation, SubtypeDerivation};
use errors::UndefinedLabel;
use errors::check_error::CheckError;
use std::collections::HashMap;
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Record, Top, Type, TypeGroup},
};
impl<Ty> Subtypecheck for Record<Ty>
where
    Ty: TypeGroup + Subtypecheck<Type = Ty> + Normalize<Ty>,
    Top<Ty>: Into<Ty>,
    Record<Ty>: Into<Ty>,
{
    type Type = Ty;
    type Term = <Ty as Subtypecheck>::Term;
    fn check_subtype(
        &self,
        sup: &Ty,
        env: Environment<Ty>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
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

impl<Ty> Kindcheck<Ty> for Record<Ty>
where
    Ty: Type + Kindcheck<Ty>,
{
    fn check_kind(&self, env: Environment<Ty>) -> Result<Kind, CheckError> {
        for (_, t) in self.records.iter() {
            t.check_kind(env.clone())?.into_star()?;
        }
        Ok(Kind::Star)
    }
}

impl<Ty> Normalize<Ty> for Record<Ty>
where
    Ty: Type + Normalize<Ty>,
    Self: Into<Ty>,
{
    fn normalize(self, env: Environment<Ty>) -> Ty {
        let mut recs_norm = HashMap::new();
        for (lb, ty) in self.records {
            let ty_norm = ty.normalize(env.clone());
            recs_norm.insert(lb, ty_norm);
        }
        Record { records: recs_norm }.into()
    }
}
