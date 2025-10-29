use crate::Eval;
use errors::eval_error::EvalError;
use grammar::DerivationRule;
use std::{collections::HashSet, rc::Rc};
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Term, UntypedLambda},
    values::UntypedLambda as UntypedLambdaVal,
};
use trace::EvalTrace;

impl<Lang> Eval for UntypedLambda<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    UntypedLambdaVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, _: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        Ok(EvalTrace::new(
            vec![],
            UntypedLambdaVal::new(&self.var, Rc::unwrap_or_clone(self.body)),
        ))
    }
    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
