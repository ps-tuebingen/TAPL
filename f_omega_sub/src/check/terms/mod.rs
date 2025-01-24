use super::{Check, Env};
use crate::{
    errors::Error,
    syntax::{terms::Term, types::Type},
};

pub mod app;
pub mod lambda;
pub mod let_exp;
pub mod pack;
pub mod pred;
pub mod record;
pub mod recordproj;
pub mod succ;
pub mod tyapp;
pub mod tylambda;
pub mod unpack;

impl Check for Term {
    type Target = Type;
    fn check(&self, env: &mut Env) -> Result<Self::Target, Error> {
        match self {
            Term::Var(v) => env.get_var(v).map_err(|knd| Error::check(knd, self)),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::TyLambda(lam) => lam.check(env),
            Term::TyApp(app) => app.check(env),
            Term::Pack(pack) => pack.check(env),
            Term::Unpack(unpack) => unpack.check(env),
            Term::Record(rec) => rec.check(env),
            Term::RecordProj(proj) => proj.check(env),
            Term::Zero => Ok(Type::Nat),
            Term::Succ(succ) => succ.check(env),
            Term::Pred(pred) => pred.check(env),
            Term::Let(lt) => lt.check(env),
        }
    }
}
