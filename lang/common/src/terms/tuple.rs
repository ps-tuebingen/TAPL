use super::Term;
use crate::{
    check::{to_check_err, Kindcheck, Typecheck},
    errors::Error,
    eval::{Eval, Normalize},
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    types::Tuple as TupleTy,
    values::Tuple as TupleVal,
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Tuple<T>
where
    T: LanguageTerm,
{
    terms: Vec<T>,
}

impl<T> Tuple<T>
where
    T: LanguageTerm,
{
    pub fn new<T1>(ts: Vec<T1>) -> Tuple<T>
    where
        T1: Into<T>,
    {
        Tuple {
            terms: ts.into_iter().map(|t| t.into()).collect(),
        }
    }
}

impl<T> Term for Tuple<T> where T: LanguageTerm {}

impl<T> SubstTerm<T> for Tuple<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst(self, v: &Var, t: &T) -> T {
        Tuple {
            terms: self.terms.into_iter().map(|t1| t1.subst(v, t)).collect(),
        }
        .into()
    }
}

impl<T> SubstType<<T as LanguageTerm>::Type> for Tuple<T>
where
    T: LanguageTerm,
    Self: Into<T>,
{
    type Target = T;
    fn subst_type(self, v: &TypeVar, ty: &<T as LanguageTerm>::Type) -> Self::Target {
        Tuple {
            terms: self
                .terms
                .into_iter()
                .map(|t| t.subst_type(v, ty))
                .collect(),
        }
        .into()
    }
}

impl<T> Typecheck for Tuple<T>
where
    T: LanguageTerm,
    TupleTy<<T as LanguageTerm>::Type>: Into<<T as LanguageTerm>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let mut tys: Vec<Self::Type> = vec![];
        let mut knd = None;
        for t in self.terms.iter() {
            let ty = t.check(&mut env.clone())?.normalize(env);
            let ty_knd = ty.check_kind(env)?;
            tys.push(ty);

            match knd {
                None => {
                    knd = Some(ty_knd);
                }
                Some(ref knd) => {
                    ty_knd.check_equal(&knd).map_err(to_check_err)?;
                }
            }
        }
        Ok(TupleTy::new::<Self::Type>(tys).into())
    }
}

impl<T> Eval for Tuple<T>
where
    T: LanguageTerm,
    TupleVal<T>: Into<<T as LanguageTerm>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        let mut vals = vec![];
        for t in self.terms.into_iter() {
            let val = t.eval(env)?;
            vals.push(val);
        }
        Ok(TupleVal::<T>::new(vals).into())
    }
}

impl<T> fmt::Display for Tuple<T>
where
    T: LanguageTerm,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut ts: Vec<String> = self.terms.iter().map(|t| t.to_string()).collect();
        ts.sort();
        write!(f, "{{ {} }}", ts.join(", "))
    }
}
