use super::{to_eval_err, Value};
use crate::syntax::Term;
use common::{
    errors::{Error, ErrorKind},
    Eval,
};

impl Eval<'_> for Term {
    type Value = Value;
    type Err = Error;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Self::Err> {
        self.eval(())
    }

    fn eval(self, _env: Self::Env) -> Result<Self::Value, Self::Err> {
        match self {
            Term::Var(v) => Err(to_eval_err(ErrorKind::FreeVariable(v))),
            Term::Lambda(lam) => lam.eval(_env),
            Term::App(app) => app.eval(_env),
            Term::TyLambda(lam) => lam.eval(_env),
            Term::TyApp(app) => app.eval(_env),
        }
    }
}
