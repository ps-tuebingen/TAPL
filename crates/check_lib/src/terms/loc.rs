use crate::{env::CheckEnvironment, to_check_err, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{Loc, Term},
    types::Reference,
};

impl<T> Typecheck for Loc<T>
where
    T: Term + Typecheck,
    <T as Typecheck>::Type: Normalize<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>
        + Kindcheck<<T as Typecheck>::Type, Env = <T as Typecheck>::Env>,
    Reference<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Env = <T as Typecheck>::Env;
    type Type = <T as Typecheck>::Type;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let loc_ty = env
            .get_loc(&self.loc)
            .map_err(to_check_err)?
            .normalize(&mut env.clone());
        loc_ty.check_kind(env)?.into_star().map_err(to_check_err)?;
        Ok(Reference::new(loc_ty).into())
    }
}
