pub mod errors;
pub mod terms;

use errors::EvalError;
use syntax::{store::Store, terms::Term, values::ValueGroup};
use trace::EvalTrace;

pub trait Eval: Sized {
    type Value: ValueGroup + Into<Self::Term>;
    type Term: Term;

    fn eval_start(self) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        self.eval(&mut Default::default())
    }

    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError>;
}
