use crate::Eval;
use errors::eval_error::EvalError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{False, Term},
    values::False as FalseVal,
};
use trace::EvalTrace;

impl<Lang> Eval for False<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    FalseVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, _: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        Ok(EvalTrace::new(vec![], FalseVal::new()))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([])
    }
}
