use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    terms::{Term, Variable},
};
use trace::{EvalStep, EvalTrace};

impl<T> Eval for Variable<T>
where
    T: Term + Eval<Term = T>,
    Variable<T>: Into<T>,
{
    type Value = <T as Eval>::Value;
    type Term = T;

    fn eval(
        self,
        ctx: &mut EvalContext<Self::Term, Self::Value>,
    ) -> Result<EvalTrace<Self::Term, Self::Value>, EvalError> {
        let body = ctx.get_name(&self.var)?;
        let mut term_res = body.clone().eval(ctx)?;
        term_res
            .steps
            .insert(0, EvalStep::subst_var(&self.var, body));
        Ok(term_res)
    }
}
