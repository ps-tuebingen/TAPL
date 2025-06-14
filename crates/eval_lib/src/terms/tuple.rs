use crate::Eval;
use syntax::{
    terms::{Term, Tuple},
    values::Tuple as TupleVal,
};
use trace::EvalTrace;

impl<T> Eval for Tuple<T>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    Tuple<T>: Into<T>,
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
        let mut old_terms = self.terms.clone();
        let mut steps = vec![];
        for (ind, t) in self.terms.into_iter().enumerate() {
            let term_res = t.eval(env)?;
            let val = term_res.val();
            vals.push(val.clone());

            let cong_vals = old_terms.clone();
            steps.extend(term_res.congruence(&move |t| {
                let mut cong_mut = cong_vals.clone();
                cong_mut[ind] = t;
                Tuple::new(cong_mut).into()
            }));

            old_terms[ind] = val.into();
        }
        let val = TupleVal::<<T as Eval>::Value>::new(vals);
        Ok(EvalTrace::new(steps, val))
    }
}
