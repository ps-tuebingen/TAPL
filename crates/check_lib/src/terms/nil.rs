use crate::{to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{Nil, Term},
    types::{List, Type},
};

impl<T, Ty> Typecheck for Nil<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: Type
        + Normalize<Ty, Env = <T as Typecheck>::Env>
        + Kindcheck<Ty, Env = <T as Typecheck>::Env>,
    List<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let ty_norm = self.ty.clone().normalize(&mut env.clone());
        ty_norm.check_kind(env)?.into_star().map_err(to_check_err)?;
        Ok(List::new(ty_norm.clone()).into())
    }
}
