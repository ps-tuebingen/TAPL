use crate::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{App, Term},
    types::{Fun, TypeGroup},
};

impl<T> Typecheck for App<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>
        + Subtypecheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
    Self: Into<T>,
    Fun<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let fun_res = self.fun.check(env.clone())?;
        let fun_ty = fun_res.ty().normalize(env.clone());
        fun_ty.check_kind(env.clone())?.into_star()?;
        let fun: Fun<<T as Typecheck>::Type> = fun_ty.into_fun()?;

        let arg_res = self.arg.check(env.clone())?;
        let arg_ty = arg_res.ty().normalize(env.clone());
        arg_ty.check_kind(env.clone())?.into_star()?;
        arg_ty.check_subtype(&(*fun.from), env.clone())?;

        let deriv_conc = Conclusion::new(env.clone(), self.clone(), *fun.to);
        let deriv = Derivation::app(deriv_conc, fun_res, arg_res);
        Ok(deriv)
    }
}
