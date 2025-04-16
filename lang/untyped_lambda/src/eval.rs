use super::{terms::Term, values::Value};
use common::{errors::Error, Eval};

impl Eval for Term {
    type Value = Value;
    type Env = ();

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::App(app) => app.eval(env),
            Term::Lambda(lam) => lam.eval(env),
        }
    }
}
