use crate::Eval;
use syntax::{
    terms::{LambdaSub, Term},
    types::Type,
    values::LambdaSub as LambdaSubVal,
};
use trace::EvalTrace;

impl<T, Ty> Eval for LambdaSub<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    LambdaSubVal<T, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    type Term = T;
    fn eval(
        self,
        _: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        Ok(LambdaSubVal::<T, Ty>::new(&self.var, self.sup_ty, *self.body).into())
    }
}
