use crate::Eval;
use errors::eval_error::EvalError;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Exception, Term},
    values::Exception as ExceptionVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Exception<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    ExceptionVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, _: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        Ok(EvalTrace::<Lang>::new(
            vec![],
            ExceptionVal::new(self.ty).into(),
        ))
    }
}
