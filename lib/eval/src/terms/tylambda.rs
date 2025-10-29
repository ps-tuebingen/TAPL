use crate::Eval;
use errors::eval_error::EvalError;
use grammar::DerivationRule;
use std::{collections::HashSet, rc::Rc};
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Term, TyLambda},
    values::TyLambda as TyLambdaVal,
};
use trace::EvalTrace;

impl<Lang> Eval for TyLambda<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    TyLambdaVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, _: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        Ok(EvalTrace::new(
            vec![],
            TyLambdaVal::new(&self.var, self.annot, Rc::unwrap_or_clone(self.term)),
        ))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
