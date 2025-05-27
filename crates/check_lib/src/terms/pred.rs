use crate::{to_check_err, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{Pred, Term},
    types::{Nat, TypeGroup},
};

impl<T> Typecheck for Pred<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: TypeGroup + Normalize<<T as Typecheck>::Type>,
    Nat<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let inner_ty = self.term.check(&mut env.clone())?.normalize(env);
        let nat = inner_ty.into_nat().map_err(to_check_err)?;
        Ok(nat.into())
    }
}
