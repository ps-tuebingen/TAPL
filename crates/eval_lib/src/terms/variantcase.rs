use crate::Eval;
use common::errors::{UndefinedLabel, ValueMismatch};
use syntax::{
    subst::SubstTerm,
    terms::{Term, VariantCase},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for VariantCase<T>
where
    T: Term + Eval<Term = T> + SubstTerm<T, Target = T> + From<<T as Eval>::Value>,
    VariantCase<T>: Into<T>,
    <T as Eval>::EvalError: From<UndefinedLabel> + From<ValueMismatch>,
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

        let var_val = bound_val.clone().into_variant()?;
        let matching = self
            .patterns
            .clone()
            .into_iter()
            .find(|pt| *pt.label == var_val.label)
            .ok_or(UndefinedLabel::new(&var_val.label))?;
        let rhs_subst = matching
            .rhs
            .subst(&matching.bound_var, &((*var_val.val).into()));
        let next_step = EvalStep::variantcase(
            VariantCase::new(bound_val, self.patterns.clone()),
            rhs_subst.clone(),
        );
        let rhs_res = rhs_subst.eval(env)?;
        let val = rhs_res.val();

        let mut steps =
            bound_res.congruence(&move |t| VariantCase::new(t, self.patterns.clone()).into());
        steps.push(next_step);
        steps.extend(rhs_res.steps.into_iter());
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, val))
    }
}
