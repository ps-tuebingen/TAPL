use crate::{Eval, errors::EvalError};
use syntax::{
    eval_context::EvalContext,
    terms::{Loc as LocT, Ref, Term},
    values::Loc,
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for Ref<T>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    LocT<T>: Into<T>,
    Ref<T>: Into<T>,
    Loc<T>: Into<<T as Eval>::Value>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut EvalContext<T, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let term_res = self.term.clone().eval(env)?;
        let term_val = term_res.val();
        let fresh_loc = env.fresh_location();
        env.save_location(fresh_loc, term_val.clone());

        let mut steps = term_res.congruence(&move |t| Ref::new(t).into());
        let val = Loc::new(fresh_loc);
        let last_step = EvalStep::reft(Ref::new(term_val), fresh_loc);
        steps.push(last_step);

        Ok(EvalTrace::new(steps, val))
    }
}
