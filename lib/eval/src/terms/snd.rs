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
    terms::{Snd, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Snd<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Snd<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let pair_val = term_val.clone().into_pair()?;
        let val = *pair_val.snd;

        let mut steps = term_res.congruence(&move |t| Snd::new(t).into());
        let last_step = EvalStep::snd(Snd::new(term_val), val.clone());
        steps.push(last_step);
        Ok(EvalTrace::<Lang>::new(steps, val))
    }
    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval(
                vec![
                    Symbol::brack(vec![
                        Symbol::sub(Symbol::Value, 1),
                        SpecialChar::Comma.into(),
                        Symbol::sub(Symbol::Value, 2),
                    ]),
                    SpecialChar::Dot.into(),
                    Keyword::Snd.into(),
                ],
                Symbol::sub(Symbol::Value, 2),
                "E-Snd",
            ),
            DerivationRule::eval_cong(
                |sym| vec![sym, SpecialChar::Dot.into(), Keyword::Snd.into()],
                "E-Snd1",
            ),
        ])
    }
}
