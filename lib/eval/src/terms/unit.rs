use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Term, Unit},
    values::Unit as UnitVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Unit<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    UnitVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, _: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        Ok(EvalTrace::new(vec![], UnitVal::new()))
    }
}
