use super::{terms::Term, values::Value};
use eval::{errors::EvalError, Eval};
use syntax::eval_context::EvalContext;
use trace::EvalTrace;

impl Eval for Term {
    type Value = Value;
    type Term = Term;

    fn eval(
        self,
        env: &mut EvalContext<Term, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::App(app) => app.eval(env),
            Term::Lambda(lam) => lam.eval(env),
        }
    }
}
