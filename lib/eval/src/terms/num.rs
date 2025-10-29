use crate::Eval;
use errors::eval_error::EvalError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Num, Term},
    values::Num as NumVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Num<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    NumVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, _: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        Ok(EvalTrace::new(vec![], NumVal::new(self.num)))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
