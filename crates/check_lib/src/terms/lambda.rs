use crate::{CheckResult, Kindcheck, Normalize, Typecheck};
use syntax::{
    env::Environment,
    terms::{Lambda, Term},
    types::{Fun, Type},
};

impl<T, Ty> Typecheck for Lambda<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: Type + Normalize<Ty> + Kindcheck<Ty>,
    <T as Typecheck>::CheckError: From<<Ty as Kindcheck<Ty>>::CheckError>,
    Fun<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Term = T;
    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<CheckResult<Self::Term, Self::Type>, Self::CheckError> {
        self.annot.check_kind(&mut env.clone())?;
        env.add_var(self.var.clone(), self.annot.clone());
        let body_ty = self
            .body
            .check(&mut env.clone())?
            .normalize(&mut env.clone());
        body_ty.check_kind(&mut env.clone())?;
        Ok(Fun::new(self.annot.clone(), body_ty).into())
    }
}
