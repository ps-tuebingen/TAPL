use crate::{errors::CheckError, Kindcheck, Normalize, Typecheck};
use derivation::{Conclusion, TypingDerivation};
use syntax::{
    env::Environment,
    terms::{Lambda, Term},
    types::{Fun, Type},
};

impl<T, Ty> Typecheck for Lambda<T, Ty>
where
    T: Term + Typecheck<Type = Ty, Term = T, Deriv = TypingDerivation<T, <T as Typecheck>::Type>>,
    Ty: Type + Normalize<Ty> + Kindcheck<Ty>,
    Fun<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
    Self: Into<T>,
{
    type Term = <T as Typecheck>::Term;
    type Type = <T as Typecheck>::Type;
    type Deriv = TypingDerivation<Self::Term, Self::Type>;

    fn check(
        &self,
        mut env: Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Deriv, CheckError> {
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
        let deriv = TypingDerivation::lambda(conc, body_res);

        Ok(deriv.into())
    }
}
