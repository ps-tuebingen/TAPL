use super::{terms::Term, types::Type, values::Value};
use check::Normalize;
use eval::{Eval, errors::EvalError};
use syntax::{env::Environment, eval_context::EvalContext};
use trace::EvalTrace;

impl Eval for Term {
    type Value = Value;
    type Term = Term;

    fn eval(
        self,
        env: &mut EvalContext<Term, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
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

impl Normalize<Type> for Type {
    fn normalize(self, _: Environment<Type>) -> Type {
        self
    }
}
