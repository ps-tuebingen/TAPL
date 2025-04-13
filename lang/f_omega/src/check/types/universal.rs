use crate::{
    check::Env,
    syntax::{kinds::Kind, types::Universal},
};
use common::{errors::Error, Typecheck};

impl<'a> Typecheck<'a> for Universal {
    type Type = Kind;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        env.add_tyvar(&self.var, &self.kind);
        self.ty.check(env)
    }
}
