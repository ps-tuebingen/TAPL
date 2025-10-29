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
    terms::{Num as NumT, Succ, Term},
    values::{Num, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Succ<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Num<Lang>: Into<Lang::Value>,
    Succ<Lang>: Into<Lang::Term>,
    NumT<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let num = term_val.into_num()?;
        let last_step = EvalStep::succ(num.num);
        let mut steps = term_res.congruence(&move |t| Succ::new(t).into());
        steps.push(last_step);
        let val = Num::<Lang>::new(num.num + 1);
        Ok(EvalTrace::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval(
                vec![
                    Keyword::Succ.into(),
                    SpecialChar::ParenO.into(),
                    Keyword::Pred.into(),
                    SpecialChar::ParenO.into(),
                    Symbol::Value,
                    SpecialChar::ParenC.into(),
                    SpecialChar::ParenC.into(),
                ],
                Symbol::Value,
                "E-SuccPred",
            ),
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::Succ.into(),
                        SpecialChar::ParenO.into(),
                        sym,
                        SpecialChar::ParenC.into(),
                    ]
                },
                "E-Succ1",
            ),
        ])
    }
}
