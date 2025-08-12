use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    terms::{LambdaSub, Term},
    types::Type,
    values::LambdaSub as LambdaSubVal,
};
use trace::EvalTrace;

impl<T, Ty> Eval for LambdaSub<T, Ty>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    Ty: Type,
    LambdaSubVal<T, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        _: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        Ok(EvalTrace::new(
            vec![],
            LambdaSubVal::<T, Ty>::new(&self.var, self.sup_ty, *self.body),
        ))
    }
}
