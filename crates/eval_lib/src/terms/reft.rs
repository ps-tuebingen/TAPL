use crate::{env::EvalEnvironment, Eval};
use syntax::{
    terms::{Ref, Term},
    values::Loc,
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for Ref<T>
where
    T: Term + Eval<Term = T>,
    <T as Eval>::Value: Into<T>,
    Ref<T>: Into<T>,
    Loc<T>: Into<<T as Eval>::Value>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let term_res = self.term.clone().eval(env)?;
        let term_val = term_res.val();
        let fresh_loc = env.fresh_location();
        env.save_location(fresh_loc, term_val);

        let mut steps = term_res.congruence(&move |t| Ref::new(t).into());
        let val = Loc::new(fresh_loc);
        let last_step = EvalStep::reft(Ref::new(term_val), fresh_loc);
        steps.push(last_step);

        Ok(EvalTrace::new(steps, val))
    }
}
