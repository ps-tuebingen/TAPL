use super::{TypedArithmetic, terms::Term, types::Type};
use check::Normalize;
use errors::eval_error::EvalError;
use eval::Eval;
use syntax::{env::Environment, eval_context::EvalContext};
use trace::EvalTrace;

impl Eval for Term {
    type Lang = TypedArithmetic;
    fn eval(self, env: &mut EvalContext<Self::Lang>) -> Result<EvalTrace<Self::Lang>, EvalError> {
        match self {
            Term::True(tru) => tru.eval(env),
            Term::False(fls) => fls.eval(env),
            Term::If(ift) => ift.eval(env),
            Term::Num(num) => num.eval(env),
            Term::Succ(succ) => succ.eval(env),
            Term::Pred(pred) => pred.eval(env),
            Term::IsZero(isz) => isz.eval(env),
        }
    }
}

impl Normalize for Type {
    type Lang = TypedArithmetic;
    fn normalize(self, _: Environment<Self::Lang>) -> Type {
        self
    }
}
