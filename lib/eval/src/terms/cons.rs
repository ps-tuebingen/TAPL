use crate::Eval;
use errors::eval_error::EvalError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, Symbol},
};
use std::{collections::HashSet, rc::Rc};
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Cons, Term},
    values::Cons as ConsVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Cons<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Self: Into<Lang::Term>,
    ConsVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let hd_res = self.head.clone().eval(env)?;
        let hd_val = hd_res.val();

        let tail_res = self.tail.clone().eval(env)?;
        let tail_val = tail_res.val();

        let val = ConsVal::<Lang>::new(hd_val, tail_val, self.ty.clone()).into();

        let ty_ = self.ty.clone();
        let mut steps = hd_res.congruence(&move |t| {
            Self::new(t, Rc::unwrap_or_clone(self.tail.clone()), ty_.clone()).into()
        });

        steps.extend(tail_res.congruence(&move |t| {
            Self::new(Rc::unwrap_or_clone(self.head.clone()), t, self.ty.clone()).into()
        }));
        Ok(EvalTrace::<Lang>::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::Cons.into(),
                        Symbol::sqbrack(Symbol::Type),
                        Symbol::paren(Symbol::comma_sep(sym, Symbol::sub(Symbol::Term, 3))),
                    ]
                },
                "E-Cons1",
            ),
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::Cons.into(),
                        Symbol::sqbrack(Symbol::Type),
                        Symbol::paren(Symbol::comma_sep(Symbol::Value, sym)),
                    ]
                },
                "E-Cons2",
            ),
        ])
    }
}
