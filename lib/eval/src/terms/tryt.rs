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
    terms::{Term, Try},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Try<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Try<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let (res_steps, res_val) = if term_val.clone().into_exception().is_ok() {
            let next_step = EvalStep::try_catch(
                Try::new(term_val.clone(), Rc::unwrap_or_clone(self.handler.clone())),
                Rc::unwrap_or_clone(self.handler.clone()),
            );
            let handler_res = self.handler.clone().eval(env)?;
            let handler_val = handler_res.val();
            let mut handler_steps = handler_res.steps;
            handler_steps.insert(0, next_step);
            (handler_steps, handler_val)
        } else {
            let next_step = EvalStep::try_succ(
                Try::new(term_val.clone(), Rc::unwrap_or_clone(self.handler.clone())),
                term_val.clone(),
            );
            (vec![next_step], term_val)
        };

        let mut steps = term_res
            .congruence(&move |t| Try::new(t, Rc::unwrap_or_clone(self.handler.clone())).into());
        steps.extend(res_steps);
        Ok(EvalTrace::<Lang>::new(steps, res_val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::Try.into(),
                        sym,
                        Keyword::With.into(),
                        Symbol::sub(Symbol::Term, 3),
                    ]
                },
                "E-Try1",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Try.into(),
                    Symbol::Value,
                    Keyword::With.into(),
                    Symbol::Term,
                ],
                Symbol::Value,
                "E-TryVal",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Try.into(),
                    Keyword::Err.into(),
                    Keyword::With.into(),
                    Symbol::Term.into(),
                ],
                Keyword::Err,
                "E-TryErr",
            ),
        ])
    }
}
