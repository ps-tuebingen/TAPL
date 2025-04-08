use super::Env;
use crate::{
    errors::Error,
    syntax::{kinds::Kind, types::Type},
};
use common::Typecheck;

pub mod existential;
pub mod fun;
pub mod opapp;
pub mod oplambda;
pub mod record;
pub mod universal;

impl<'a> Typecheck<'a> for Type {
    type Type = Kind;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        match self {
            Type::Var(v) => env
                .get_tyvar(v)
                .map_err(|knd| Error::kinding(knd, &v.as_str())),
            Type::Fun(fun) => fun.check(env),
            Type::Universal(uni) => uni.check(env),
            Type::OpLambda(lam) => lam.check(env),
            Type::OpApp(app) => app.check(env),
            Type::Existential(ex) => ex.check(env),
            Type::Record(rec) => rec.check(env),
            Type::Bool => Ok(Kind::Star),
            Type::Unit => Ok(Kind::Star),
            Type::Nat => Ok(Kind::Star),
        }
    }
}
