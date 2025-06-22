use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, TypingDerivation};
use syntax::{
    env::Environment,
    terms::{Term, Tuple},
    types::Tuple as TupleTy,
};

impl<T> Typecheck for Tuple<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    TupleTy<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
        let mut tys: Vec<Self::Type> = vec![];
        let mut ress = vec![];
        let mut knd = None;
        for t in self.terms.iter() {
            let t_res = t.check(env.clone())?;
            let t_ty = t_res.ty().normalize(env.clone());
            ress.push(t_res);
            let ty_knd = t_ty.check_kind(env.clone())?;
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

        let conc = Conclusion::new(env, self.clone(), TupleTy::new::<Self::Type>(tys));
        let deriv = TypingDerivation::tuple(conc, ress);
        Ok(deriv.into())
    }
}
