use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
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
    Fun<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;

    fn check(
        &self,
        mut env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Derivation<Self::Term, Self::Type>, CheckError<Self::Type>> {
        self.annot.check_kind(env.clone())?;
        env.add_var(self.var.clone(), self.annot.clone());
        let body_res = self.body.check(env.clone())?;
        let body_ty = body_res.ty().normalize(env.clone());
        body_ty.check_kind(env.clone())?;

        let conc = Conclusion::new(
            env.clone(),
            self.clone(),
            Fun::new(self.annot.clone(), body_ty).into(),
        );
        let deriv = Derivation::lambda(conc, body_res);

        Ok(deriv)
    }
}
