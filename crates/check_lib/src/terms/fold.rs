use crate::{CheckResult, Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use syntax::{
    env::Environment,
    kinds::Kind,
    subst::SubstType,
    terms::{Fold, Term},
    types::{Mu, TypeGroup},
};

impl<T, Ty> Typecheck for Fold<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty> + SubstType<Ty, Target = Ty>,
    Mu<Ty>: Into<Ty>,
    <T as Typecheck>::CheckError:
        From<TypeMismatch> + From<KindMismatch> + From<<Ty as Kindcheck<Ty>>::CheckError>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<CheckResult<Self::Term, Self::Type>, Self::CheckError> {
        let mu_ty = self.ty.clone().normalize(&mut env.clone()).into_mu()?;
        env.add_tyvar_kind(mu_ty.var.clone(), Kind::Star);
        mu_ty.ty.check_kind(&mut env.clone())?.into_star()?;

        let mu_subst = mu_ty
            .ty
            .clone()
            .subst_type(&mu_ty.var.clone(), &(mu_ty.into()));
        let term_ty = self
            .term
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        term_ty.check_kind(env)?.into_star()?;
        term_ty.check_equal(&mu_subst)?;
        Ok(self.ty.clone())
    }
}
