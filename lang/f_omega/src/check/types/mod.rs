use super::{CheckKind, Env};
use crate::{
    errors::Error,
    syntax::{kinds::Kind, types::Type},
};

pub mod existential;
pub mod fun;
pub mod opapp;
pub mod oplambda;
pub mod record;
pub mod universal;

impl CheckKind for Type {
    fn check_kind(&self, env: &mut Env) -> Result<Kind, Error> {
        match self {
            Type::Var(v) => env
                .get_tyvar(v)
                .map_err(|knd| Error::kinding(knd, &v.as_str())),
            Type::Fun(fun) => fun.check_kind(env),
            Type::Universal(uni) => uni.check_kind(env),
            Type::OpLambda(lam) => lam.check_kind(env),
            Type::OpApp(app) => app.check_kind(env),
            Type::Existential(ex) => ex.check_kind(env),
            Type::Record(rec) => rec.check_kind(env),
        }
    }
}
