use super::{errors::Error, terms::Term, types::Type, values::Value};
use check::Normalize;
use eval::Eval;
use syntax::env::Environment;

impl Eval for Term {
    type Env = ();
    type Value = Value;
    type EvalError = Error;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::TyLambda(lam) => lam.eval(env),
            Term::TyApp(app) => app.eval(env),
        }
    }
}

impl Normalize<Type> for Type {
    fn normalize(self, _: Environment<Type>) -> Type {
        self
    }
}
