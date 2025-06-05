use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Term, Tuple},
    types::Tuple as TupleTy,
};

impl<T> Typecheck for Tuple<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
    TupleTy<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let mut tys: Vec<Self::Type> = vec![];
        let mut ress = vec![];
        let mut knd = None;
        for t in self.terms.iter() {
            let t_res = t.check(&mut env.clone())?;
            let t_ty = t_res.ty().normalize(&mut env.clone());
            ress.push(t_res);
            let ty_knd = t_ty.check_kind(env)?;
            tys.push(t_ty);

            match knd {
                None => {
                    knd = Some(ty_knd);
                }
                Some(ref knd) => {
                    ty_knd.check_equal(knd)?;
                }
            }
        }

        let conc = Conclusion::new(env.clone(), self.clone(), TupleTy::new::<Self::Type>(tys));
        let deriv = Derivation::tuple(conc, ress);
        Ok(deriv)
    }
}
