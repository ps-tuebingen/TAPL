use crate::Eval;
use syntax::{
    terms::{Right, Term},
    types::Type,
    values::Right as RightVal,
};
use trace::EvalTrace;

impl<T, Ty> Eval for Right<T, Ty>
where
    T: Term + Eval,
    Ty: Type,
    RightVal<<T as Eval>::Value, Ty>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let right_val = self.right_term.eval(env)?;
        Ok(RightVal::<<T as Eval>::Value, Ty>::new(right_val, self.ty.clone()).into())
    }
}
