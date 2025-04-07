use super::{CheckType, Env};
use crate::{
    errors::Error,
    syntax::{terms::Term, types::Type},
};

pub mod app;
pub mod bool;
pub mod lambda;
pub mod pack;
pub mod record;
pub mod record_proj;
pub mod tyapp;
pub mod tylambda;
pub mod unpack;

impl CheckType for Term {
    fn check_type(&self, env: &mut Env) -> Result<Type, Error> {
        match self {
            Term::Var(v) => env.get_var(v).map_err(|knd| Error::check(knd, &v.as_str())),
            Term::Lambda(lam) => lam.check_type(env),
            Term::App(app) => app.check_type(env),
            Term::TyLambda(lam) => lam.check_type(env),
            Term::TyApp(app) => app.check_type(env),
            Term::Pack(pack) => pack.check_type(env),
            Term::Unpack(unpack) => unpack.check_type(env),
            Term::Record(rec) => rec.check_type(env),
            Term::RecordProj(proj) => proj.check_type(env),
            Term::True(tru) => tru.check_type(env),
            Term::False(fls) => fls.check_type(env),
            Term::If(ift) => ift.check_type(env),
            Term::Unit => Ok(Type::Unit),
        }
    }
}
