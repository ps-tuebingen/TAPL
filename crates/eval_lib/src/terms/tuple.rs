use crate::Eval;
use syntax::{
    terms::{Term, Tuple},
    values::Tuple as TupleVal,
};
use trace::EvalTrace;

impl<T> Eval for Tuple<T>
where
    T: Term + Eval,
    TupleVal<<T as Eval>::Value>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;
    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let mut vals = vec![];
        for t in self.terms.into_iter() {
            let val = t.eval(env)?;
            vals.push(val);
        }
        Ok(TupleVal::<<T as Eval>::Value>::new(vals).into())
    }
}
