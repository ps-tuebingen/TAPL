use crate::Eval;
use common::errors::ValueMismatch;
use syntax::{
    terms::{Term, Unfold},
    types::Type,
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<T, Ty> Eval for Unfold<T, Ty>
where
    T: Term + Eval<Term = T>,
    Unfold<T, Ty>: Into<T>,
    Ty: Type,
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
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let term_fold = term_val.into_fold()?;

        let last_step = EvalStep::unfoldfold(
            Unfold::new(self.ty.clone(), term_val),
            *term_fold.val.clone(),
        );
        let mut steps = term_res.congruence(&move |t| Unfold::new(self.ty.clone(), t).into());
        steps.push(last_step);
        Ok(EvalTrace::new(steps, *term_fold.val))
    }
}
