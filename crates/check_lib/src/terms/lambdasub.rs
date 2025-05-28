use crate::{env::CheckEnvironment, Kindcheck, Normalize, Typecheck};
use common::errors::Error;
use syntax::{
    terms::{LambdaSub, Term},
    types::{ForallBounded, Type},
};

impl<T, Ty> Typecheck for LambdaSub<T, Ty>
where
    T: Term + Typecheck<Type = Ty>,
    Ty: Type
        + Kindcheck<Ty, Env = <T as Typecheck>::Env>
        + Normalize<Ty, Env = <T as Typecheck>::Env>,
    ForallBounded<<T as Typecheck>::Type>: Into<<T as Typecheck>::Type>,
{
    type Type = <T as Typecheck>::Type;
    type Env = <T as Typecheck>::Env;

    fn check(&self, env: &mut Self::Env) -> Result<Self::Type, Error> {
        let sup_norm = self.sup_ty.clone().normalize(&mut env.clone());
        let sup_kind = sup_norm.check_kind(&mut env.clone())?;
        env.add_tyvar_super(self.var.clone(), sup_norm.clone());
        env.add_tyvar_kind(self.var.clone(), sup_kind.clone());
        let term_ty = self.body.check(&mut env.clone())?.normalize(env);
        Ok(ForallBounded::new(&self.var, self.sup_ty.clone(), term_ty).into())
    }
}
