use crate::{Eval, errors::EvalError};
use syntax::{
    eval_context::EvalContext,
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
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
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
