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
    terms::{Tail, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Tail<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let cons_val = term_val.clone().into_cons()?;

        let val = *cons_val.head;
        let last_step = EvalStep::tail(Self::new(term_val, self.ty.clone()), val.clone());
        let mut steps = term_res.congruence(&move |t| Self::new(t, self.ty.clone()).into());
        steps.push(last_step);
        Ok(EvalTrace::<Lang>::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval(
                vec![
                    Keyword::Tail.into(),
                    Symbol::sqbrack(Symbol::Type),
                    Symbol::paren(vec![
                        Keyword::Cons.into(),
                        Symbol::sqbrack(Symbol::Type),
                        Symbol::paren(Symbol::comma_sep(
                            Symbol::sub(Symbol::Value, 1),
                            Symbol::sub(Symbol::Value, 2),
                        )),
                    ]),
                ],
                Symbol::Value,
                "E-TailCons",
            ),
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::Tail.into(),
                        Symbol::sqbrack(Symbol::Type),
                        Symbol::paren(sym),
                    ]
                },
                "E-Tail1",
            ),
        ])
    }
}
