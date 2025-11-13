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
    terms::{Num as NumT, Pred, Term},
    values::{Num, ValueGroup},
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Pred<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Self: Into<Lang::Term>,
    NumT<Lang>: Into<Lang::Term>,
    Num<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let num = term_val.into_num()?;
        let val = Num::<Lang>::new(num.num - 1);
        let mut steps = term_res.congruence(&move |t| Self::new(t).into());
        let last_step = EvalStep::pred(num.num);
        steps.push(last_step);
        Ok(EvalTrace::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| vec![Keyword::Pred.into(), Symbol::paren(sym)],
                "E-Pred1",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Pred.into(),
                    Symbol::paren(vec![Keyword::Succ.into(), Symbol::paren(Symbol::Value)]),
                ],
                Symbol::Value,
                "E-PredSucc",
            ),
            DerivationRule::eval(
                vec![Keyword::Pred.into(), Symbol::paren(0)],
                0,
                "E-PredZero",
            ),
        ])
    }
}
