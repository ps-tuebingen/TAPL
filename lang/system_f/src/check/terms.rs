use super::{to_check_err, Env};
use crate::{syntax::Term, types::Type};
use common::errors::{Error, ErrorKind};
use common::Typecheck;

impl<'a> Typecheck<'a> for Term {
    type Type = Type;
    type Env = &'a mut Env;

    fn check_start(&self) -> Result<Self::Type, Error> {
        self.check(&mut Default::default())
    }

    fn check(&self, env: Self::Env) -> Result<Self::Type, Error> {
        match self {
            Term::Var(v) => env
                .vars
                .get(v)
                .cloned()
                .ok_or(to_check_err(ErrorKind::FreeVariable(v.clone()))),
            Term::Lambda(lam) => lam.check(env),
            Term::App(app) => app.check(env),
            Term::TyLambda(lam) => lam.check(env),
            Term::TyApp(app) => app.check(env),
        }
    }
}
