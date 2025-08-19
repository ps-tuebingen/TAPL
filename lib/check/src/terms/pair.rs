use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Pair, Term},
    types::Product,
};

impl<T> Typecheck for Pair<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Product<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        let fst_res = self.fst.check(env.clone())?;
        let fst_ty = fst_res.ret_ty().normalize(env.clone());

        let snd_res = self.snd.check(env.clone())?;
        let snd_ty = snd_res.ret_ty().normalize(env.clone());

        let fst_knd = fst_ty.check_kind(env.clone())?;
        let snd_knd = snd_ty.check_kind(env.clone())?;
        fst_knd.check_equal(&snd_knd)?;

        let conc = TypingConclusion::new(env, self.clone(), Product::new(fst_ty, snd_ty));
        let deriv = TypingDerivation::pair(conc, fst_res, snd_res);
        Ok(deriv.into())
    }
}
