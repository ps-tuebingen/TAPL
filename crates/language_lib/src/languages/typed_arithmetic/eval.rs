use super::{errors::Error, terms::Term, types::Type, values::Value};
use check::Normalize;
use eval::Eval;
use syntax::env::Environment;
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
