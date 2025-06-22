use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Assign, Term},
    types::{TypeGroup, Unit as UnitTy},
};

impl<T> Typecheck for Assign<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type:
        TypeGroup + Normalize<<T as Typecheck>::Type> + Kindcheck<<T as Typecheck>::Type>,
    UnitTy<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError<Self::Type>> {
        let lhs_res = self.lhs.check(env.clone())?;
        let lhs_ty = lhs_res.ty().normalize(env.clone());
        lhs_ty.check_kind(env.clone())?.into_star()?;
        let lhs_ref = lhs_ty.into_ref()?;

        let rhs_res = self.rhs.check(env.clone())?;
        let rhs_ty = rhs_res.ty().normalize(env.clone());
        rhs_ty.check_kind(env.clone())?.into_star()?;
        lhs_ref.ty.check_equal(&rhs_ty)?;

        let conc = Conclusion::new(env.clone(), self.clone(), UnitTy::new());
        let deriv = Derivation::assign(conc, lhs_res, rhs_res);

        Ok(deriv)
    }
}
