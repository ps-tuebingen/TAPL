use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    kinds::Kind,
    subst::SubstType,
    terms::{Fold, Term},
    types::{Mu, TypeGroup},
};

impl<T, Ty> Typecheck for Fold<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty> + SubstType<Ty, Target = Ty>,
    Mu<Ty>: Into<Ty>,
    <T as Typecheck>::CheckError:
        From<TypeMismatch> + From<KindMismatch> + From<<Ty as Kindcheck<Ty>>::CheckError>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        mut env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let mu_ty = self.ty.clone().normalize(env.clone()).into_mu()?;
        env.add_tyvar_kind(mu_ty.var.clone(), Kind::Star);
        mu_ty.ty.check_kind(env.clone())?.into_star()?;

        let mu_subst = mu_ty
            .ty
            .clone()
            .subst_type(&mu_ty.var.clone(), &(mu_ty.into()));
        let term_res = self.term.check(env.clone())?;
        let term_ty = term_res.ty().normalize(env.clone());
        term_ty.check_kind(env.clone())?.into_star()?;
        term_ty.check_equal(&mu_subst)?;

        let conc = Conclusion::new(env, self.clone(), self.ty.clone());
        let deriv = Derivation::fold(conc, term_res);
        Ok(deriv)
    }
}
