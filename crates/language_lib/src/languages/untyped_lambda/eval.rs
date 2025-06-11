use super::{errors::Error, terms::Term, values::Value};
use eval::Eval;
use trace::EvalTrace;

impl Eval for Term {
    type Value = Value;
    type Term = Term;
    type Env = ();
    type EvalError = Error;

    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::App(app) => app.eval(env),
            Term::Lambda(lam) => lam.eval(env),
        }
    }
}
