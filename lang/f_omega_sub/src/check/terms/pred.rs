use crate::{
    check::{Check, Env},
    errors::Error,
    syntax::{terms::Pred, types::Type},
};
use common::Eval;

impl Check for Pred {
    type Target = Type;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        let t_ty = self.term.check(&mut env.clone())?.eval(env)?;
        t_ty.check_equal(&Type::Nat)?;
        Ok(Type::Nat)
    }
}
