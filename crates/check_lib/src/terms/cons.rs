use crate::{Kindcheck, Normalize, Typecheck};
use common::errors::{KindMismatch, TypeMismatch};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Cons, Term},
    types::{List, TypeGroup},
};

impl<T, Ty> Typecheck for Cons<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T>,
    Ty: TypeGroup + Normalize<Ty> + Kindcheck<Ty>,
    List<Ty>: Into<Ty>,
    <T as Typecheck>::CheckError:
        From<TypeMismatch> + From<KindMismatch> + From<<Ty as Kindcheck<Ty>>::CheckError>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        let ty_norm = self.ty.clone().normalize(env.clone());
        let hd_res = self.head.check(env.clone())?;
        let hd_ty = hd_res.ty().normalize(env.clone());
        hd_ty.check_equal(&ty_norm)?;
        hd_ty.check_kind(env.clone())?.into_star()?;

        let tl_res = self.tail.check(env.clone())?;
        let tl_ty = tl_res.ty().normalize(env.clone());
        tl_ty.check_kind(env.clone())?.into_star()?;
        let list_ty: Self::Type = List::new(ty_norm).into();
        tl_ty.check_equal(&list_ty)?;

        let conc = Conclusion::new(env, self.clone(), list_ty.clone());
        let deriv = Derivation::cons(conc, hd_res, tl_res);
        Ok(deriv)
    }
}
