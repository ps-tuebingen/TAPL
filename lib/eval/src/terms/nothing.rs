use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Nothing, Term},
    values::Nothing as NothingVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Nothing<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    NothingVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, _: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        Ok(EvalTrace::new(vec![], NothingVal::<Lang>::new(self.ty)))
    }
}
