use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Conclusion, Derivation, TypingDerivation};
use errors::check_error::CheckError;
use std::collections::HashMap;
use syntax::{
    env::Environment,
    terms::{Record, Term},
    types::Record as RecordTy,
};

impl<T> Typecheck for Record<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    RecordTy<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        let mut recs = HashMap::new();
        let mut ress = Vec::new();
        let mut rec_knd = None;
        for (lb, t) in self.records.iter() {
            let term_res = t.check(env.clone())?;
            let ty = term_res.ret_ty().normalize(env.clone());
            ress.push(term_res);

            let ty_knd = ty.check_kind(env.clone())?;
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

        let conc = Conclusion::new(env, self.clone(), RecordTy::new(recs));
        let deriv = TypingDerivation::record(conc, ress);
        Ok(deriv.into())
    }
}
