pub mod env;
pub mod terms;

use env::EvalEnvironment;
use syntax::values::ValueGroup;

pub trait Eval: Sized {
    type EvalError: std::error::Error;
    type Env: EvalEnvironment<Self::Value, EvalError: Into<<Self as Eval>::EvalError>>;
    type Value: ValueGroup;

    fn eval_start(self) -> Result<Self::Value, Self::EvalError> {
        self.eval(&mut Self::Env::default())
    }

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError>;
}
