use crate::{
    check::Env,
    errors::Error,
    syntax::{terms::Pred, types::Type},
};
use common::Eval;
use common::Typecheck;

impl<'a> Typecheck<'a> for Pred {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;
    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let t_ty = self.term.check(&mut env.clone())?.eval(env)?;
        t_ty.check_equal(&Type::Nat)?;
        Ok(Type::Nat)
    }
}
