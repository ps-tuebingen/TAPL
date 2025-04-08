use super::{Error, Value};
use crate::syntax::Term;
use common::Eval;

impl<'a> Eval<'a> for Term {
    type Value = Value;
    type Error = Error;
    type Env = ();

    fn eval(self, env: Self::Env) -> Result<Value, Error> {
        match self {
            Term::Var(v) => Err(Error::FreeVar(v)),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::Unit(u) => u.eval(env),
            Term::True => Ok(Value::True),
            Term::False => Ok(Value::False),
            Term::Succ(s) => s.eval(env),
            Term::Pred(p) => p.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::IsZero(isz) => isz.eval(env),
            Term::Const(i) => Ok(Value::Const(i)),
            Term::Error(err) => err.eval(env),
            Term::Try(tr) => tr.eval(env),
            Term::TryWithVal(tr) => tr.eval(env),
            Term::Raise(r) => r.eval(env),
        }
    }
}
