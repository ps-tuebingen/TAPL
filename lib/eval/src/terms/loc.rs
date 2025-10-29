use crate::Eval;
use errors::eval_error::EvalError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Loc, Term},
    values::Loc as LocVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Loc<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    LocVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, _: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        Ok(EvalTrace::<Lang>::new(vec![], LocVal::new(self.loc).into()))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
