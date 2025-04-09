use super::{errors::Error, Env};
use crate::{syntax::Term, types::Type};
use common::Typecheck;

impl<'a> Typecheck<'a> for Term {
    type Type = Type;
    type Err = Error;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Self::Err> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Self::Err> {
        match self {
            Term::Var(v) => env.vars.get(v).cloned().ok_or(Error::FreeVar(v.clone())),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::TyLambda(lam) => lam.check(env),
            Term::TyApp(app) => app.check(env),
        }
    }
}
