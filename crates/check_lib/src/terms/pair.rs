use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Pair, Term},
    types::Product,
};

impl<T> Typecheck for Pair<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
    Product<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let fst_res = self.fst.check(&mut env.clone())?;
        let fst_ty = fst_res.ty().normalize(&mut env.clone());

        let snd_res = self.snd.check(&mut env.clone())?;
        let snd_ty = snd_res.ty().normalize(&mut env.clone());

        let fst_knd = fst_ty.check_kind(&mut env.clone())?;
        let snd_knd = snd_ty.check_kind(env)?;
        fst_knd.check_equal(&snd_knd)?;

        let conc = Conclusion::new(env.clone(), self.clone(), Product::new(fst_ty, snd_ty));
        let deriv = Derivation::pair(conc, fst_res, snd_res);
        Ok(deriv)
    }
}
