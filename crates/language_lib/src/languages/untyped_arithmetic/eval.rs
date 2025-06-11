use super::{errors::Error, terms::Term, values::Value};
use eval::Eval;
use trace::EvalTrace;

impl Eval for Term {
    type Value = Value;
    type Term = Term;
    type Env = ();
    type EvalError = Error;

    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
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
