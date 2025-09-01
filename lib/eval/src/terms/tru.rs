use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Term, True},
    values::True as TrueVal,
};
use trace::EvalTrace;

impl<Lang> Eval for True<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    TrueVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, _: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        Ok(EvalTrace::new(vec![], TrueVal::new()))
    }
}
