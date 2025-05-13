use super::{terms::Term, values::Value};
use common::{errors::Error, eval::Eval};

impl Eval for Term {
    type Value = Value;
    type Env = ();

    fn eval_start(self) -> Result<Self::Value, Error> {
        self.eval(&mut ())
    }

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Error> {
        match self {
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::Num(num) => num.eval(env),
            Term::IsZero(isz) => isz.eval(env),
            Term::Succ(succ) => succ.eval(env),
            Term::Pred(pred) => pred.eval(env),
            Term::If(ift) => ift.eval(env),
        }
    }
}
