use crate::Eval;
use errors::eval_error::EvalError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, Symbol},
};
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Something, Term},
    values::Something as SomethingVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Something<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Self: Into<Lang::Term>,
    SomethingVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let val = SomethingVal::<Lang>::new(term_val);
        let steps = term_res.congruence(&move |t| Self::new(t).into());
        Ok(EvalTrace::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::eval_cong(
            |sym| {
                vec![
                    Keyword::Something.into(),
                    Symbol::sqbrack(Symbol::Term),
                    Symbol::paren(sym),
                ]
            },
            "E-Something1",
        )])
    }
}
