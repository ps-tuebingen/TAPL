use crate::{to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use std::collections::HashMap;
use syntax::{
    terms::{Record, Term},
    types::Record as RecordTy,
};

impl<T> Typecheck for Record<T>
where
    T: Term + Typecheck,
    RecordTy<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
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
                    knd.check_equal(&ty_knd).map_err(to_check_err)?;
                }
            }
        }
        Ok(RecordTy::new(recs).into())
    }
}
