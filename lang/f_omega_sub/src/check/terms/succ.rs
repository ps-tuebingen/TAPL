use super::to_check_err;
use crate::{
    check::Env,
    syntax::{terms::Succ, types::Type},
};
use common::{errors::Error, Eval, Typecheck};

impl<'a> Typecheck<'a> for Succ {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        let t_ty = self.term.check(&mut env.clone())?.eval(env)?;
        t_ty.check_equal(&Type::Nat).map_err(to_check_err)?;
        Ok(Type::Nat)
    }
}
