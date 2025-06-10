use crate::Eval;
use common::errors::{ValueKind, ValueMismatch};
use syntax::{
    subst::SubstTerm,
    terms::{ListCase, Term},
    values::{Value, ValueGroup},
};
use trace::EvalTrace;

impl<T> Eval for ListCase<T>
where
    T: Term + Eval + SubstTerm<T, Target = T> + From<<T as Eval>::Value>,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let bound_val = self.bound_term.eval(env)?;
        if bound_val.clone().into_nil().is_ok() {
            self.nil_rhs.eval(env)
        } else if let Ok(cons) = bound_val.clone().into_cons() {
            self.cons_rhs
                .subst(&self.cons_fst, &((*cons.head).into()))
                .subst(&self.cons_rst, &((*cons.tail).into()))
                .eval(env)
        } else {
            Err(ValueMismatch::new(bound_val.knd(), ValueKind::List).into())
        }
    }
}
