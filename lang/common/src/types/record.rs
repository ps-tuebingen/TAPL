use super::Type;
use crate::{
    check::{to_subty_err, Subtypecheck},
    errors::{Error, ErrorKind},
    language::LanguageType,
    subst::SubstType,
    Label, TypeVar,
};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record<Ty>
where
    Ty: Type,
{
    pub records: HashMap<Label, Ty>,
}

impl<Ty> Record<Ty>
where
    Ty: Type,
{
    pub fn new<Ty1>(recs: HashMap<Label, Ty1>) -> Record<Ty>
    where
        Ty1: Into<Ty>,
    {
        Record {
            records: recs.into_iter().map(|(lb, ty)| (lb, ty.into())).collect(),
        }
    }
}

impl<Ty> Type for Record<Ty> where Ty: Type {}

impl<Ty> SubstType<Ty> for Record<Ty>
where
    Ty: Type + SubstType<Ty, Target = Ty>,
    Self: Into<Ty>,
{
    type Target = Ty;
    fn subst_type(self, v: &TypeVar, ty: &Ty) -> Self::Target {
        Record {
            records: self
                .records
                .into_iter()
                .map(|(lb, ty1)| (lb, ty1.subst_type(v, ty)))
                .collect(),
        }
        .into()
    }
}

impl<Ty> Subtypecheck<Ty> for Record<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Subtypecheck<Ty>>::Env;
    fn check_supertype(&self, sub: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if let Ok(_) = sub.clone().into_bot() {
            return Ok(());
        }

        let sub_rec = sub.clone().into_record().map_err(to_subty_err)?;
        for (lb, ty) in sub_rec.records.iter() {
            let self_ty = self
                .records
                .get(lb)
                .ok_or(to_subty_err(ErrorKind::UndefinedLabel(lb.clone())))?;
            ty.check_supertype(self_ty, &mut env.clone())?;
        }
        Ok(())
    }

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        if let Ok(_) = sup.clone().into_top() {
            return Ok(());
        }

        let sup_rec = sup.clone().into_record().map_err(to_subty_err)?;
        for (lb, ty) in self.records.iter() {
            let sup_ty = sup_rec
                .records
                .get(lb)
                .ok_or(to_subty_err(ErrorKind::UndefinedLabel(lb.clone())))?;
            ty.check_subtype(sup_ty, &mut env.clone())?;
        }
        Ok(())
    }
}

impl<Ty> fmt::Display for Record<Ty>
where
    Ty: Type,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&String, &Ty)> = self.records.iter().collect();
        recs.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
        write!(
            f,
            "{{ {} }}",
            recs.iter()
                .map(|(lb, ty)| format!("{lb} : {ty}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
