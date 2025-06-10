use crate::{Kindcheck, Normalize, Subtypecheck};
use common::errors::{KindMismatch, TypeMismatch, UndefinedLabel};
use std::collections::HashMap;
use syntax::{
    env::Environment,
    kinds::Kind,
    types::{Record, Type, TypeGroup},
};
impl<Ty> Subtypecheck<Ty> for Record<Ty>
where
    Ty: TypeGroup + Subtypecheck<Ty> + Normalize<Ty>,
    <Ty as Subtypecheck<Ty>>::CheckError: From<TypeMismatch> + From<UndefinedLabel>,
{
    type CheckError = <Ty as Subtypecheck<Ty>>::CheckError;

    fn check_subtype(&self, sup: &Ty, env: Environment<Ty>) -> Result<(), Self::CheckError> {
        if sup.clone().into_top().is_ok() {
            return Ok(());
        }

        let sup_norm = sup.clone().normalize(env.clone());
        let sup_rec = sup_norm.into_record()?;
        for (lb, ty) in sup_rec.records.iter() {
            let sub_ty = self.records.get(lb).ok_or(UndefinedLabel::new(lb))?;
            sub_ty.check_subtype(ty, env.clone())?;
        }
        Ok(())
    }
}

impl<Ty> Kindcheck<Ty> for Record<Ty>
where
    Ty: Type + Kindcheck<Ty>,
    <Ty as Kindcheck<Ty>>::CheckError: From<KindMismatch>,
{
    type CheckError = <Ty as Kindcheck<Ty>>::CheckError;

    fn check_kind(&self, env: Environment<Ty>) -> Result<Kind, Self::CheckError> {
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
