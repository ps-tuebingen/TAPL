use crate::{Eval, errors::EvalError};
use syntax::{
    eval_context::EvalContext,
    terms::{Lambda, Term},
    types::Type,
    values::Lambda as LambdaVal,
};
use trace::EvalTrace;

impl<T, Ty> Eval for Lambda<T, Ty>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    Ty: Type,
    LambdaVal<T, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;

    fn eval(
        self,
        _: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        Ok(EvalTrace::new(
            vec![],
            LambdaVal::new(&self.var, self.annot, *self.body),
        ))
    }
}
