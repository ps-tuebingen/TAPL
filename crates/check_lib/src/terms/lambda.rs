use crate::{Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, Derivation};
use syntax::{
    env::Environment,
    terms::{Lambda, Term},
    types::{Fun, Type},
};

impl<T, Ty> Typecheck for Lambda<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T>,
    Ty: Type + Normalize<Ty> + Kindcheck<Ty>,
    <T as Typecheck>::CheckError: From<<Ty as Kindcheck<Ty>>::CheckError>,
    Fun<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, Self::CheckError> {
        self.annot.check_kind(&mut env.clone())?;
        env.add_var(self.var.clone(), self.annot.clone());
        let body_res = self.body.check(&mut env.clone())?;
        let body_ty = body_res.ty().normalize(&mut env.clone());
        body_ty.check_kind(&mut env.clone())?;

        let conc = Conclusion::new(
            env.clone(),
            self.clone(),
            Fun::new(self.annot.clone(), body_ty).into(),
        );
        let deriv = Derivation::lambda(conc, body_res);

        Ok(deriv)
    }
}
