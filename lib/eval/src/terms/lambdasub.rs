use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{LambdaSub, Term},
    values::LambdaSub as LambdaSubVal,
};
use trace::EvalTrace;

impl<Lang> Eval for LambdaSub<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    LambdaSubVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, _: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        Ok(EvalTrace::new(
            vec![],
            LambdaSubVal::<Lang>::new(&self.var, self.sup_ty, *self.body),
        ))
    }
}
