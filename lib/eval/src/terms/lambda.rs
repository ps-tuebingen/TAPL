use crate::Eval;
use errors::eval_error::EvalError;
use grammar::DerivationRule;
use std::{collections::HashSet, rc::Rc};
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Lambda, Term},
    values::Lambda as LambdaVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Lambda<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    LambdaVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, _: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        Ok(EvalTrace::new(
            vec![],
            LambdaVal::new(&self.var, self.annot, Rc::unwrap_or_clone(self.body)),
        ))
    }
    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
