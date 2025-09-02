use super::{SystemF, terms::Term, types::Type};
use check::Normalize;
use errors::eval_error::EvalError;
use eval::Eval;
use syntax::env::Environment;
use syntax::eval_context::EvalContext;
use trace::EvalTrace;

impl Eval for Term {
    type Lang = SystemF;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::TyLambda(lam) => lam.eval(env),
            Term::TyApp(app) => app.eval(env),
        }
    }
}

impl Normalize for Type {
    type Lang = SystemF;
    fn normalize(self, _: Environment<Self::Lang>) -> Type {
        self
    }
}
