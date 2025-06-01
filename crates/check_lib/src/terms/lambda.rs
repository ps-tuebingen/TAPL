use crate::{CheckEnvironment, Kindcheck, Normalize, Typecheck};
use syntax::{
    terms::{Lambda, Term},
    types::{Fun, Type},
};

impl<T, Ty> Typecheck for Lambda<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: Type
        + Normalize<Ty, Env = <T as Typecheck>::Env>
        + Kindcheck<Ty, Env = <T as Typecheck>::Env, CheckError = <T as Typecheck>::CheckError>,
    Fun<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type CheckError = <T as Typecheck>::CheckError;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Self::CheckError> {
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
