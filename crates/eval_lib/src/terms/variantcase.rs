use crate::Eval;
use common::errors::{UndefinedLabel, ValueMismatch};
use syntax::{
    subst::SubstTerm,
    terms::{Term, VariantCase},
    values::ValueGroup,
};
use trace::EvalTrace;

impl<T> Eval for VariantCase<T>
where
    T: Term + Eval + SubstTerm<T, Target = T> + From<<T as Eval>::Value>,
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
        let bound_val = self.bound_term.eval(env)?;
        let var_val = bound_val.into_variant()?;
        let matching = self
            .patterns
            .into_iter()
            .find(|pt| *pt.label == var_val.label)
            .ok_or(UndefinedLabel::new(&var_val.label))?;
        matching
            .rhs
            .subst(&matching.bound_var, &((*var_val.val).into()))
            .eval(env)
    }
}
