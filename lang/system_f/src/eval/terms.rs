use super::{errors::Error, Value};
use crate::syntax::Term;
use common::Eval;

impl Eval<'_> for Term {
    type Value = Value;
    type Error = Error;
    type Env = ();
    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Error> {
        match self {
            Term::Var(v) => Err(Error::FreeVar(v)),
            Term::Lambda(lam) => lam.eval(_env),
            Term::App(app) => app.eval(_env),
            Term::TyLambda(lam) => lam.eval(_env),
            Term::TyApp(app) => app.eval(_env),
        }
    }
}
