use super::Type;
use crate::{
    check::{to_kind_err, to_subty_err, CheckEnvironment, Kindcheck, Subtypecheck},
    errors::{Error, ErrorKind},
    eval::Normalize,
    kinds::Kind,
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

    fn check_subtype(&self, sup: &Ty, env: &mut Self::Env) -> Result<(), Error> {
        println!("checking suptype {self}<:{sup}");
        if let Ok(_) = sup.clone().into_top() {
            return Ok(());
        }

        println!("converting {sup} to record (checking subtype record)");
        let sup_rec = sup.clone().into_record().map_err(to_subty_err)?;
        for (lb, ty) in sup_rec.records.iter() {
            let sub_ty = self
                .records
                .get(lb)
                .ok_or(to_subty_err(ErrorKind::UndefinedLabel(lb.clone())))?;
            sub_ty.check_subtype(ty, &mut env.clone())?;
        }
        Ok(())
    }
}

impl<Ty> Kindcheck<Ty> for Record<Ty>
where
    Ty: LanguageType,
{
    type Env = <Ty as Kindcheck<Ty>>::Env;
    fn check_kind(&self, env: &mut Self::Env) -> Result<Kind, Error> {
        for (_, t) in self.records.iter() {
            t.check_kind(&mut env.clone())?
                .into_star()
                .map_err(to_kind_err)?;
        }
        Ok(Kind::Star)
    }
}

impl<Ty> Normalize<Ty> for Record<Ty>
where
    Ty: LanguageType,
    Self: Into<Ty>,
{
    fn normalize(self) -> Ty {
        let mut recs_norm = HashMap::new();
        for (lb, ty) in self.records {
            let ty_norm = ty.normalize();
            recs_norm.insert(lb, ty_norm);
        }
        Record { records: recs_norm }.into()
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
            "{{{}}}",
            recs.iter()
                .map(|(lb, ty)| format!("{lb}:{ty}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
