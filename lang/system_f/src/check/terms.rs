use super::{errors::Error, Check, Env};
use crate::{syntax::Term, types::Type};

impl Check for Term {
    fn check(self, env: &mut Env) -> Result<Type, Error> {
        match self {
            Term::Var(v) => env.vars.get(&v).cloned().ok_or(Error::FreeVar(v)),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::TyLambda(lam) => lam.check(env),
            Term::TyApp(app) => app.check(env),
        }
    }
}
