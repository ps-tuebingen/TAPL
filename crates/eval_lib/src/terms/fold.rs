use crate::Eval;
use syntax::{
    terms::{Fold, Term},
    types::Type,
    values::Fold as FoldVal,
};

impl<T, Ty> Eval for Fold<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    FoldVal<<T as Eval>::Value, Ty>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let term_val = self.term.eval(env)?;
        Ok(FoldVal::<<T as Eval>::Value, Ty>::new(self.ty, term_val).into())
    }
}
