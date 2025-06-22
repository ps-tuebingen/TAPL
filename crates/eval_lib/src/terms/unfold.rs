use crate::{Eval, errors::EvalError};

use syntax::{
    store::Store,
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
{
    type Value = <T as Eval>::Value;

    type Term = T;
    fn eval(
        self,
        env: &mut Store<Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let term_fold = term_val.clone().into_fold()?;

        let last_step = EvalStep::unfoldfold(
            Unfold::new(self.ty.clone(), term_val),
            *term_fold.val.clone(),
        );
        let mut steps = term_res.congruence(&move |t| Unfold::new(self.ty.clone(), t).into());
        steps.push(last_step);
        Ok(EvalTrace::<T, <T as Eval>::Value>::new(
            steps,
            *term_fold.val,
        ))
    }
}
