use crate::{Kindcheck, Normalize, Typecheck};
use derivations::{Derivation, TypingConclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{Term, TyLambda},
    types::Forall,
};

impl<T> Typecheck for TyLambda<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Forall<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;

    fn check(
        &self,
        mut env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError> {
        env.add_tyvar_kind(self.var.clone(), self.annot.clone());
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ret_ty().normalize(env.clone());
        let term_knd = term_ty.check_kind(env.clone())?;
        self.annot.check_equal(&term_knd)?;
        let ty = Forall::new(&self.var, self.annot.clone(), term_ty);
        let conc = TypingConclusion::new(env, self.clone(), ty);
        let deriv = TypingDerivation::tylambda(conc, term_res);
        Ok(deriv.into())
    }
}
