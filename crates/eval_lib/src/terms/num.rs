use crate::Eval;
use syntax::{
    terms::{Num, Term},
    values::Num as NumVal,
};
use trace::EvalTrace;

impl<T> Eval for Num<T>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    NumVal<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Env = <T as Eval>::Env;

    type Term = T;
    fn eval(
        self,
        _: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        Ok(EvalTrace::new(vec![], NumVal::new(self.num)))
    }
}
