use crate::Eval;
use errors::eval_error::EvalError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Right, Term},
    values::Right as RightVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Right<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Right<Lang>: Into<Lang::Term>,
    RightVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let right_res = self.right_term.eval(env)?;
        let right_val = right_res.val();
        let val = RightVal::<Lang>::new(right_val, self.ty.clone());
        let steps = right_res.congruence(&move |t| Right::new(t, self.ty.clone()).into());
        Ok(EvalTrace::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::eval_cong(
            |sym| {
                vec![
                    Keyword::Right.into(),
                    SpecialChar::ParenO.into(),
                    sym,
                    SpecialChar::ParenC.into(),
                    Keyword::As.into(),
                    Symbol::Type,
                ]
            },
            "E-Right",
        )])
    }
}
