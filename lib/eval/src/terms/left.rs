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
    terms::{Left, Term},
    values::Left as LeftVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Left<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Left<Lang>: Into<Lang::Term>,
    LeftVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let left_res = self.left_term.eval(env)?;
        let left_val = left_res.val();
        let val = LeftVal::<Lang>::new(left_val, self.ty.clone());
        let steps = left_res.congruence(&move |t| Left::new(t, self.ty.clone()).into());
        Ok(EvalTrace::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::eval_cong(
            |sym| {
                vec![
                    Keyword::Left.into(),
                    SpecialChar::ParenO.into(),
                    sym,
                    SpecialChar::ParenC.into(),
                    Keyword::As.into(),
                    Symbol::Type,
                ]
            },
            "E-Left1",
        )])
    }
}
