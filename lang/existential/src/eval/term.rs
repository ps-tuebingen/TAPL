use super::{Eval, Value};
use crate::{errors::Error, terms::Term};

impl Eval for Term {
    fn eval(self) -> Result<Value, Error> {
        match self {
            Term::Var(var) => var.eval(),
            Term::Unit => Ok(Value::Unit),
            Term::Lambda(lam) => lam.eval(),
            Term::App(app) => app.eval(),
            Term::Pack(pack) => pack.eval(),
            Term::Unpack(unpack) => unpack.eval(),
            Term::Record(rec) => rec.eval(),
            Term::RecordProj(proj) => proj.eval(),
            Term::Zero(zero) => zero.eval(),
            Term::Succ(succ) => succ.eval(),
            Term::Pred(pred) => pred.eval(),
            Term::IsZero(isz) => isz.eval(),
            Term::True(tru) => tru.eval(),
            Term::False(fls) => fls.eval(),
            Term::If(ift) => ift.eval(),
        }
    }
}
