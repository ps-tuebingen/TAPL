use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Ascribe, Term},
    types::TypeGroup,
};

impl<T, Ty> Typecheck for Ascribe<T, Ty>
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
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let t_res = self.term.check(&mut env.clone())?;
        let t_ty = t_res.ty().normalize(&mut env.clone());
        let asc_norm = self.ty.clone().normalize(&mut env.clone());
        let t_kind = t_ty.check_kind(&mut env.clone())?;
        let ty_kind = self.ty.check_kind(env)?;
        t_kind.check_equal(&ty_kind)?;
        asc_norm.check_equal(&t_ty)?;

        let conc = Conclusion::new(env.clone(), self.clone(), self.ty.clone());
        let deriv = Derivation::ascribe(conc, t_res);
        Ok(deriv)
    }
}
