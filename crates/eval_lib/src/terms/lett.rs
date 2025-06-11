use crate::Eval;
use syntax::{
    subst::SubstTerm,
    terms::{Let, Term},
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for Let<T>
where
    T: Term + Eval<Term = T> + SubstTerm<T, Target = T> + From<<T as Eval>::Value>,
    Let<T>: Into<T>,
{
    type Env = <T as Eval>::Env;
    type Value = <T as Eval>::Value;
    type EvalError = <T as Eval>::EvalError;

    type Term = T;
    fn eval(
        self,
        env: &mut Self::Env,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, Self::EvalError> {
        let bound_res = self.bound_term.eval(env)?;
        let bound_val = bound_res.val();

        let term_subst = self
            .in_term
            .clone()
            .subst(&self.var, &bound_val.clone().into());
        let subst_step = EvalStep::lett(
            Let::new(&self.var, bound_val, *self.in_term.clone()),
            term_subst.clone(),
        );

        let mut steps =
            bound_res.congruence(&move |t| Let::new(&self.var, t, *self.in_term.clone()).into());
        steps.push(subst_step);
        let term_res = term_subst.eval(env)?;
        let val = term_res.val();
        steps.extend(term_res.steps);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, val))
    }
}
