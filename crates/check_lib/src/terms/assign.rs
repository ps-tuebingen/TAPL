use crate::{CheckResult, Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Assign, Term},
    types::{TypeGroup, Unit as UnitTy},
};

impl<T> Typecheck for Assign<T>
where
    T: Term + Typecheck<Term = T>,
    <T as Typecheck>::Type: TypeGroup
        + Normalize<<T as Typecheck>::Type>
        + Kindcheck<<T as Typecheck>::Type, CheckError = <T as Typecheck>::CheckError>,
    UnitTy<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    <T as Typecheck>::CheckError: From<TypeMismatch> + From<KindMismatch>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<CheckResult<Self::Term, Self::Type>, Self::CheckError> {
        let lhs_res = self.lhs.check(&mut env.clone())?;
        let lhs_ty = lhs_res.ty.normalize(&mut env.clone());
        lhs_ty.check_kind(&mut env.clone())?.into_star()?;
        let lhs_ref = lhs_ty.into_ref()?;

        let rhs_res = self.rhs.check(&mut env.clone())?;
        let rhs_ty = rhs_res.ty.normalize(&mut env.clone());
        rhs_ty.check_kind(&mut env.clone())?.into_star()?;
        lhs_ref.ty.check_equal(&rhs_ty)?;

        let conc = Conclusion::new(env.clone(), self.clone(), UnitTy::new());
        let deriv = Derivation::assign(conc, lhs_res.derivation, rhs_res.derivation);

        Ok(CheckResult::new(UnitTy::new(), deriv))
    }
}
