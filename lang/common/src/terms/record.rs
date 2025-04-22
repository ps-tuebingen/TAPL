use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::Error,
    eval::Eval,
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    types::Record as RecordTy,
    values::Record as RecordVal,
    Label, TypeVar, Var,
};
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record<T>
where
    T: LanguageTerm,
{
    records: HashMap<Label, T>,
}

impl<T> Record<T>
where
    T: LanguageTerm,
{
    pub fn new<T1>(recs: HashMap<Label, T1>) -> Record<T>
    where
        T1: Into<T>,
    {
        Record {
            records: recs.into_iter().map(|(lb, t)| (lb, t.into())).collect(),
        }
    }
}

impl<T> Term for Record<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Record<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Record {
            records: self
                .records
                .into_iter()
                .map(|(lb, t1)| (lb, t1.subst(v, t)))
                .collect(),
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Record<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Record {
            records: self
                .records
                .into_iter()
                .map(|(lb, t)| (lb, t.subst_type(v, ty)))
                .collect(),
        }
        .into()
    }
}

impl<T> Typecheck for Record<T>
where
    T: LanguageTerm,
    RecordTy<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        println!("checking record term");
        let mut recs = HashMap::new();
        let mut rec_knd = None;
        for (lb, t) in self.records.iter() {
            println!("checking type for {lb}");
            let ty = t.check(&mut env.clone())?;
            let ty_knd = ty.check_kind(env)?;
            recs.insert(lb.clone(), ty);
            match rec_knd {
                None => {
                    rec_knd = Some(ty_knd);
                }
                Some(ref knd) => {
                    knd.check_equal(&ty_knd).map_err(to_check_err)?;
                }
            }
        }
        Ok(RecordTy::new(recs).into())
    }
}

impl<T> Eval for Record<T>
where
    T: LanguageTerm,
    RecordVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let mut recs: HashMap<Label, Self::Value> = HashMap::new();
        for (lb, t) in self.records.into_iter() {
            let val = t.eval(env)?;
            recs.insert(lb, val);
        }
        Ok(RecordVal::<T>::new::<Self::Value>(recs).into())
    }
}

impl<T> fmt::Display for Record<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut recs: Vec<(&Label, &T)> = self.records.iter().collect();
        recs.sort_by(|(lb1, _), (lb2, _)| lb1.cmp(lb2));
        write!(
            f,
            "{{ {} }}",
            recs.iter()
                .map(|(lb, t)| format!("{lb} = {t}"))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}
