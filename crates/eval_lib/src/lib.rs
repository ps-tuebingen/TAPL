pub mod env;
pub mod terms;

use env::EvalEnvironment;
use syntax::{terms::Term, values::ValueGroup};
use trace::EvalTrace;

pub trait Eval: Sized {
    type EvalError: std::error::Error;
    type Env: EvalEnvironment<Self::Value, EvalError: Into<<Self as Eval>::EvalError>>;
    type Value: ValueGroup + Into<Self::Term>;
    type Term: Term;

    fn eval_start(self) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        self.eval(&mut Self::Env::default())
    }

    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError>;
}
