use crate::{Kindcheck, Normalize, Subtypecheck, Typecheck};
use derivations::{Conclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{App, Term},
    types::{Fun, TypeGroup},
};

impl<T> Typecheck for App<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type>
        + Subtypecheck<<T as Typecheck>::Type>,
    Self: Into<T>,
    Fun<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(&self, env: Environment<<T as Typecheck>::Type>) -> Result<Self::Deriv, CheckError> {
        let fun_res = self.fun.check(env.clone())?;
        let fun_ty = fun_res.ty().normalize(env.clone());
        fun_ty.check_kind(env.clone())?.into_star()?;
        let fun: Fun<<T as Typecheck>::Type> = fun_ty.into_fun()?;

        let arg_res = self.arg.check(env.clone())?;
        let arg_ty = arg_res.ty().normalize(env.clone());
        arg_ty.check_kind(env.clone())?.into_star()?;
        arg_ty.check_subtype(&(*fun.from), env.clone())?;

        let deriv_conc = Conclusion::new(env.clone(), self.clone(), *fun.to);
        let deriv = TypingDerivation::app(deriv_conc, fun_res, arg_res);
        Ok(deriv)
    }
}
