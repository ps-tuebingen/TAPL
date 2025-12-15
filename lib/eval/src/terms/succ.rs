use crate::Eval;
use errors::{ValueMismatch, eval_error::EvalError};
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Num as NumT, Succ, Term},
    values::{Num, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Succ<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Num<Lang>: Into<Lang::Value>,
    Self: Into<Lang::Term>,
    NumT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let num = term_val.clone().into_num().ok_or(ValueMismatch::new(
            term_val.to_string(),
            "Number".to_string(),
        ))?;
        let last_step = EvalStep::succ(num.num, self.span);
        let mut steps = term_res.congruence(&move |t| Self::new(t, self.span).into());
        steps.push(last_step);
        let val = Num::<Lang>::new(num.num + 1, self.span);
        Ok(EvalTrace::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval(
                vec![
                    Keyword::Succ.into(),
                    Symbol::paren(vec![
                        Keyword::Pred.into(),
                        Symbol::paren(Symbol::Value),
                        SpecialChar::ParenC.into(),
                    ]),
                ],
                Symbol::Value,
                "E-SuccPred",
            ),
            DerivationRule::eval_cong(
                |sym| vec![Keyword::Succ.into(), Symbol::paren(sym)],
                "E-Succ1",
            ),
        ])
    }
}
