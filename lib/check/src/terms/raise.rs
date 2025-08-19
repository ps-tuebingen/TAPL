use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Raise, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Raise<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        let ex_norm = self.exception_ty.clone().normalize(env.clone());
        let cont_norm = self.cont_ty.clone().normalize(env.clone());

        let ex_knd = ex_norm.check_kind(env.clone())?;
        self.cont_ty.check_kind(env.clone())?;

        let err_res = self.exception.check(env.clone())?;
        let err_ty = err_res.ret_ty().normalize(env.clone());
        let err_knd = err_ty.check_kind(env.clone())?;

        ex_knd.check_equal(&err_knd)?;
        ex_norm.check_equal(&err_ty)?;

        let conc = TypingConclusion::new(env, self.clone(), cont_norm.clone());
        let deriv = TypingDerivation::raise(conc, err_res);

        Ok(deriv.into())
    }
}
