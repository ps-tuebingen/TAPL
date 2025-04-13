use super::Env;
use crate::{terms::Let, types::Type};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Let {
    type Type = Type;
    type Env = &'a mut Env;
    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }
    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        let bound_ty = self.bound_term.check(&mut env.clone())?;
        env.insert(self.var.clone(), bound_ty);
        self.in_term.check(env)
    }
}
