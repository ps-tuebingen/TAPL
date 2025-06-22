use crate::Eval;
use common::errors::{ValueKind, ValueMismatch};
use syntax::{
    store::Store,
    subst::SubstTerm,
    terms::{ListCase, Term},
    values::{Value, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for ListCase<T>
where
    T: Term + Eval<Term = T> + SubstTerm<T, Target = T> + From<<T as Eval>::Value>,
    ListCase<T>: Into<T>,
    <T as Eval>::EvalError: From<ValueMismatch>,
{
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let bound_res = self.bound_term.eval(env)?;
        let bound_val = bound_res.val();

        let (res_steps, val) = if bound_val.clone().into_nil().is_ok() {
            let next_step = EvalStep::listcase_nil(
                ListCase::new(
                    bound_val,
                    *self.nil_rhs.clone(),
                    &self.cons_fst,
                    &self.cons_rst,
                    *self.cons_rhs.clone(),
                ),
                *self.nil_rhs.clone(),
            );
            let nil_res = self.nil_rhs.clone().eval(env)?;
            let nil_val = nil_res.val();
            let mut steps = nil_res.steps;
            steps.insert(0, next_step);
            (steps, nil_val)
        } else if let Ok(cons) = bound_val.clone().into_cons() {
            let cons_subst = self
                .cons_rhs
                .clone()
                .subst(&self.cons_fst, &((*cons.head).into()))
                .subst(&self.cons_rst, &((*cons.tail).into()));
            let next_step = EvalStep::listcase_cons(
                ListCase::new(
                    bound_val,
                    *self.nil_rhs.clone(),
                    &self.cons_fst,
                    &self.cons_rst,
                    *self.cons_rhs.clone(),
                ),
                cons_subst.clone(),
            );
            let cons_res = cons_subst.eval(env)?;
            let cons_val = cons_res.val();
            let mut steps = cons_res.steps;
            steps.insert(0, next_step);
            (steps, cons_val)
        } else {
            return Err(ValueMismatch::new(bound_val.knd(), ValueKind::List).into());
        };

        let mut steps = bound_res.congruence(&move |t| {
            ListCase::new(
                t,
                *self.nil_rhs.clone(),
                &self.cons_fst,
                &self.cons_rst,
                *self.cons_rhs.clone(),
            )
            .into()
        });
        steps.extend(res_steps);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, val))
    }
}
