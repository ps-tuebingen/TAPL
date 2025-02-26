use crate::{
    check::{Check, Env},
    errors::Error,
    eval::Eval,
    syntax::{terms::Succ, types::Type},
};

impl Check for Succ {
    type Target = Type;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        let t_ty = self.term.check(&mut env.clone())?.eval(env)?;
        t_ty.check_equal(&Type::Nat)?;
        Ok(Type::Nat)
    }
}
