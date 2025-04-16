use crate::{terms::Term, values::Value};
use common::{errors::Error, eval::Eval};

impl Eval for Term {
    type Value = Value;
    type Env = ();
    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Error> {
        match self {
            Term::Var(var) => var.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Succ(succ) => succ.eval(env),
            Term::Pred(pred) => pred.eval(env),
            Term::Lambda(lam) => lam.eval(env),
            Term::App(app) => app.eval(env),
            Term::LambdaSub(lam) => lam.eval(env),
            Term::TyApp(app) => app.eval(env),
            Term::Pack(pack) => pack.eval(env),
            Term::Unpack(unpack) => unpack.eval(env),
            Term::Record(rec) => rec.eval(env),
            Term::Projection(proj) => proj.eval(env),
        }
    }
}
