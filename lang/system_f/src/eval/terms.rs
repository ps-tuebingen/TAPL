use super::{errors::Error, Eval, Value};
use crate::syntax::Term;

impl Eval for Term {
    fn eval(self) -> Result<Value, Error> {
        match self {
            Term::Var(v) => Err(Error::FreeVar(v)),
            Term::Lambda(lam) => lam.eval(),
            Term::App(app) => app.eval(),
            Term::TyLambda(lam) => lam.eval(),
            Term::TyApp(app) => app.eval(),
        }
    }
}
