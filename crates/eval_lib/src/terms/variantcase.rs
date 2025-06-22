use crate::{errors::EvalError, Eval};
use common::errors::UndefinedLabel;
use syntax::{
    store::Store,
    subst::SubstTerm,
    terms::{Term, VariantCase},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for VariantCase<T>
where
    T: Term + Eval<Term = T> + SubstTerm<T, Target = T> + From<<T as Eval>::Value>,
    VariantCase<T>: Into<T>,
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
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
        steps.extend(rhs_res.steps);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(steps, val))
    }
}
