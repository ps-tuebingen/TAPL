use super::{errors::Error, terms::Term, values::Value};
use eval::Eval;

impl Eval for Term {
    type Value = Value;
    type Env = ();
    type EvalError = Error;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::App(app) => app.eval(env),
            Term::Lambda(lam) => lam.eval(env),
        }
    }
}
