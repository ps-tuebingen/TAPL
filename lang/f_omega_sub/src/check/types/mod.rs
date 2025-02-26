use super::{Check, Env};
use crate::{
    errors::Error,
    syntax::{kinds::Kind, types::Type},
};

pub mod existential;
pub mod fun;
pub mod opapp;
pub mod oplambda;
pub mod record;
pub mod subtype;
pub mod universal;

impl Check for Type {
    type Target = Kind;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        match self {
            Type::Var(v) => {
                let ty = env.get_tyvar(v).map_err(|knd| Error::kind(knd, self))?;
                ty.check(env)
            }
            Type::Top(knd) => Ok(knd.clone()),
            Type::Fun(fun) => fun.check(env),
            Type::Universal(uni) => uni.check(env),
            Type::OpLambda(lam) => lam.check(env),
            Type::OpApp(app) => app.check(env),
            Type::Existential(ex) => ex.check(env),
            Type::Record(rec) => rec.check(env),
            Type::Nat => Ok(Kind::Star),
        }
    }
}
