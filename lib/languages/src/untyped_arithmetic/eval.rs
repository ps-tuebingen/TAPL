use super::{UntypedArithmetic, terms::Term};
use errors::eval_error::EvalError;
use eval::Eval;
use syntax::eval_context::EvalContext;
use trace::EvalTrace;

impl Eval for Term {
    type Lang = UntypedArithmetic;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
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
