use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Raise, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Raise<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
    <T as Typecheck>::CheckError:
        From<TypeMismatch> + From<KindMismatch> + From<<Ty as Kindcheck<Ty>>::CheckError>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let ex_norm = self.exception_ty.clone().normalize(env.clone());
        let cont_norm = self.cont_ty.clone().normalize(env.clone());

        let ex_knd = ex_norm.check_kind(env.clone())?;
        self.cont_ty.check_kind(env.clone())?;

        let err_res = self.exception.check(env.clone())?;
        let err_ty = err_res.ty().normalize(env.clone());
        let err_knd = err_ty.check_kind(env.clone())?;

        ex_knd.check_equal(&err_knd)?;
        ex_norm.check_equal(&err_ty)?;

        let conc = Conclusion::new(env, self.clone(), cont_norm.clone());
        let deriv = Derivation::raise(conc, err_res);

        Ok(deriv)
    }
}
