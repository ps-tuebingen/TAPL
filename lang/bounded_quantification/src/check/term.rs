use super::{Check, Env};
use crate::{
    errors::{Error, ErrorKind},
    syntax::Term,
    types::Type,
};

impl Check for Term {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        match self {
            Term::Var(v) => env
                .vars
                .get(v)
                .cloned()
                .ok_or(Error::check(ErrorKind::FreeVar(v.clone()), &v.as_str())),
            Term::Const(c) => c.check(env),
            Term::Succ(s) => s.check(env),
            Term::Pred(p) => p.check(env),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::LambdaSub(lam) => lam.check(env),
            Term::TyApp(app) => app.check(env),
            Term::Pack(pack) => pack.check(env),
            Term::Unpack(unpack) => unpack.check(env),
            Term::Record(rec) => rec.check(env),
            Term::Projection(proj) => proj.check(env),
        }
    }
}
