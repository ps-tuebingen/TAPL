use crate::{Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, TypingDerivation};
use errors::check_error::CheckError;
use syntax::{
    env::Environment,
    terms::{ListCase, Term},
    types::TypeGroup,
};

impl<T> Typecheck for ListCase<T>
where
    T: Term + Typecheck<Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    <T as Typecheck>::Type:
        TypeGroup + Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(
        &self,
        mut env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Deriv, CheckError> {
        let bound_res = self.bound_term.check(env.clone())?;
        let bound_ty = bound_res.ty().normalize(env.clone());
        bound_ty.check_kind(env.clone())?.into_star()?;
        let bound_list = bound_ty.clone().into_list()?;

        let nil_res = self.nil_rhs.check(env.clone())?;
        let nil_ty = nil_res.ty().normalize(env.clone());
        let nil_kind = nil_ty.check_kind(env.clone())?;

        env.add_var(self.cons_fst.clone(), *bound_list.ty);
        env.add_var(self.cons_rst.clone(), bound_ty);
        let cons_res = self.cons_rhs.check(env.clone())?;
        let cons_ty = nil_res.ty().normalize(env.clone());
        let cons_kind = cons_ty.check_kind(env.clone())?;

        nil_kind.check_equal(&cons_kind)?;
        nil_ty.check_equal(&cons_ty)?;

        let conc = Conclusion::new(env.clone(), self.clone(), cons_ty);
        let deriv = TypingDerivation::listcase(conc, nil_res, cons_res);

        Ok(deriv)
    }
}
