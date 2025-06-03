use crate::{Kindcheck, Normalize, Typecheck};
use syntax::{
    env::Environment,
    terms::{Exception, Term},
    types::Type,
};

impl<T, Ty> Typecheck for Exception<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: Type + Normalize<Ty> + Kindcheck<Ty>,
    <T as Typecheck>::CheckError: From<<Ty as Kindcheck<Ty>>::CheckError>,
{
    type Type = <T as Typecheck>::Type;

    type CheckError = <T as Typecheck>::CheckError;

    fn check(
        &self,
        env: &mut Environment<<T as Typecheck>::Type>,
    ) -> Result<Self::Type, Self::CheckError> {
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        ty_norm.check_kind(env)?;
        Ok(ty_norm)
    }
}
