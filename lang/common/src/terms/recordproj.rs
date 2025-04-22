use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::{Error, ErrorKind},
    eval::{to_eval_err, Eval},
    language::{LanguageTerm, LanguageType, LanguageValue},
    subst::{SubstTerm, SubstType},
    Label, TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RecordProj<T>
where
    T: LanguageTerm,
{
    record: Box<T>,
    label: Label,
}

impl<T> RecordProj<T>
where
    T: LanguageTerm,
{
    pub fn new<T1>(t: T1, lb: &str) -> RecordProj<T>
    where
        T1: Into<T>,
    {
        RecordProj {
            record: Box::new(t.into()),
            label: lb.to_owned(),
        }
    }
}

impl<T> Term for RecordProj<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for RecordProj<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        RecordProj {
            record: Box::new(self.record.subst(v, t)),
            label: self.label,
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for RecordProj<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        RecordProj {
            record: Box::new(self.record.subst_type(v, ty)),
            label: self.label,
        }
        .into()
    }
}

impl<T> Typecheck for RecordProj<T>
where
    T: LanguageTerm,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let term_ty = self.record.check(env)?;
        term_ty.check_kind(env)?;
        let rec_ty = term_ty.into_record().map_err(to_check_err)?;
        rec_ty
            .records
            .get(&self.label)
            .ok_or(to_check_err(ErrorKind::UndefinedLabel(self.label.clone())))
            .cloned()
    }
}

impl<T> Eval for RecordProj<T>
where
    T: LanguageTerm,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let term_val = self.record.eval(env)?;
        let rec_val = term_val.into_record().map_err(to_eval_err)?;
        rec_val
            .records
            .get(&self.label)
            .ok_or(to_eval_err(ErrorKind::UndefinedLabel(self.label.clone())))
            .cloned()
    }
}

impl<T> fmt::Display for RecordProj<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.record, self.label)
    }
}
