use crate::Eval;
use syntax::{
    terms::{Left, Term},
    types::Type,
    values::Left as LeftVal,
};

impl<T, Ty> Eval for Left<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    LeftVal<<T as Eval>::Value, Ty>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, env: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        let left_val = self.left_term.eval(env)?;
        Ok(LeftVal::<<T as Eval>::Value, Ty>::new(left_val, self.ty).into())
    }
}
