use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use std::collections::HashMap;
use syntax::{
    terms::{Record, Term},
    types::Record as RecordTy,
};

impl<T> Typecheck for Record<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<
            <T as Typecheck>::Type,
            Env = <T as Typecheck>::Env,
            CheckError = <T as Typecheck>::CheckError,
        >,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
    RecordTy<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
        let mut recs = HashMap::new();
        let mut rec_knd = None;
        for (lb, t) in self.records.iter() {
            let ty = t.check(&mut env.clone())?.normalize(&mut env.clone());
            let ty_knd = ty.check_kind(env)?;
            recs.insert(lb.clone(), ty);
            match rec_knd {
                None => {
                    rec_knd = Some(ty_knd);
                }
                Some(ref knd) => {
                    knd.check_equal(&ty_knd)?;
                }
            }
        }
        Ok(RecordTy::new(recs).into())
    }
}
