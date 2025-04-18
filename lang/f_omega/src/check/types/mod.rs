use super::{to_kind_err, Env};
use crate::syntax::{kinds::Kind, types::Type};
use common::{errors::Error, Typecheck};

pub mod existential;
pub mod fun;
pub mod opapp;
pub mod oplambda;
pub mod record;
pub mod universal;

impl<'a> Typecheck<'a> for Type {
    type Type = Kind;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        match self {
            Type::Var(v) => env.get_tyvar(v).map_err(to_kind_err),
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
