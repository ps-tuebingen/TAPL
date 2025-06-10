use crate::Eval;
use syntax::{
    terms::{Loc, Term},
    values::Loc as LocVal,
};

impl<T> Eval for Loc<T>
where
    T: Term + Eval,
    LocVal<T>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    fn eval(self, _: &mut Self::Env) -> Result<Self::Value, Self::EvalError> {
        Ok(LocVal::new(self.loc).into())
    }
}
