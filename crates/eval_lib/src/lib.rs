pub mod errors;
pub mod terms;

use errors::EvalError;
use syntax::{
    eval_context::EvalContext, program::Program, terms::Term, types::Type, values::ValueGroup,
};
use trace::EvalTrace;

pub trait Eval: Sized {
    type Value: ValueGroup + Into<Self::Term>;
    type Term: Term;

    fn eval_start(self) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        self.eval(&mut Default::default())
    }

    fn eval(
        self,
        env: &mut EvalContext<Self::Term, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError>;
}

pub fn eval_main<T, Ty>(prog: Program<T, Ty>) -> Result<EvalTrace<T, <T as Eval>::Value>, EvalError>
where
    T: Term + Eval<Term = T>,
    Ty: Type,
{
    let mut ctx = EvalContext::<T, <T as Eval>::Value>::from_prog(&prog);
    prog.main.eval(&mut ctx)
}
