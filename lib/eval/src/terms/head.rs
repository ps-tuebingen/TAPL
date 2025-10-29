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
    terms::{Head, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Head<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Head<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let cons_val = term_val.clone().into_cons()?;

        let last_step =
            EvalStep::head(Head::new(term_val, self.ty.clone()), *cons_val.head.clone());
        let mut steps = term_res.congruence(&move |t| Head::new(t, self.ty.clone()).into());
        steps.push(last_step);

        Ok(EvalTrace::<Lang>::new(steps, *cons_val.head))
    }
    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::Head.into(),
                        SpecialChar::SqBrackO.into(),
                        Symbol::Type,
                        SpecialChar::SqBrackC.into(),
                        SpecialChar::ParenO.into(),
                        sym,
                        SpecialChar::ParenC.into(),
                    ]
                },
                "E-Head1",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Head.into(),
                    SpecialChar::SqBrackO.into(),
                    Symbol::Type,
                    SpecialChar::SqBrackC.into(),
                    SpecialChar::ParenO.into(),
                    Keyword::Cons.into(),
                    SpecialChar::SqBrackO.into(),
                    Symbol::Type,
                    SpecialChar::SqBrackC.into(),
                    SpecialChar::ParenO.into(),
                    Symbol::sub(Symbol::Value, 1),
                    SpecialChar::Comma.into(),
                    Symbol::sub(Symbol::Value, 2),
                    SpecialChar::ParenC.into(),
                ],
                vec![Symbol::sub(Symbol::Value, 1)],
                "E-HeadCons",
            ),
        ])
    }
}
