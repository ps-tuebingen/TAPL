use super::{to_eval_err, Value};
use crate::terms::Term;
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
            Term::Var(ref var) => Err(to_eval_err(ErrorKind::FreeVariable(var.clone()))),
            Term::Unit => Ok(Value::Unit),
            Term::Lambda(lam) => lam.eval(_env),
            Term::App(app) => app.eval(_env),
            Term::Pack(pack) => pack.eval(_env),
            Term::Unpack(unpack) => unpack.eval(_env),
            Term::Record(rec) => rec.eval(_env),
            Term::RecordProj(proj) => proj.eval(_env),
            Term::Zero(zero) => zero.eval(_env),
            Term::Succ(succ) => succ.eval(_env),
            Term::Pred(pred) => pred.eval(_env),
            Term::IsZero(isz) => isz.eval(_env),
            Term::True(tru) => tru.eval(_env),
            Term::False(fls) => fls.eval(_env),
            Term::If(ift) => ift.eval(_env),
            Term::Fix(fix) => fix.eval(_env),
        }
    }
}
