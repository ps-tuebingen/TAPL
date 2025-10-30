use crate::Eval;
use errors::{ValueMismatch, eval_error::EvalError};
use grammar::{
    DerivationRule,
    symbols::{Keyword, Symbol},
};
use std::{collections::HashSet, rc::Rc};
use syntax::{eval_context::EvalContext, language::Language, terms::If, values::ValueGroup};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for If<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
    If<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let cond_res = self.if_cond.eval(env)?;
        let cond_val = cond_res.val();
        let (next_step, branch_res) = if cond_val.clone().into_true().is_ok() {
            (
                EvalStep::if_true(
                    If::new(
                        cond_val,
                        Rc::unwrap_or_clone(self.then_term.clone()),
                        Rc::unwrap_or_clone(self.else_term.clone()),
                    ),
                    Rc::unwrap_or_clone(self.then_term.clone()),
                ),
                self.then_term.clone().eval(env)?,
            )
        } else if cond_val.clone().into_false().is_ok() {
            (
                EvalStep::if_false(
                    If::new(
                        cond_val,
                        Rc::unwrap_or_clone(self.then_term.clone()),
                        Rc::unwrap_or_clone(self.else_term.clone()),
                    ),
                    Rc::unwrap_or_clone(self.else_term.clone()),
                ),
                self.else_term.clone().eval(env)?,
            )
        } else {
            return Err(
                ValueMismatch::new(cond_val.to_string(), "Boolean Value".to_owned()).into(),
            );
        };
        let branch_val = branch_res.val();

        let mut steps = cond_res.congruence(&move |t| {
            If::new(
                t,
                Rc::unwrap_or_clone(self.then_term.clone()),
                Rc::unwrap_or_clone(self.else_term.clone()),
            )
            .into()
        });
        steps.push(next_step);
        steps.extend(branch_res.steps);

        Ok(EvalTrace::<Lang>::new(steps, branch_val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::If.into(),
                        sym,
                        Symbol::brack(Symbol::sub(Symbol::Term, 3)),
                        Keyword::Else.into(),
                        Symbol::brack(Symbol::sub(Symbol::Term, 4)),
                    ]
                },
                "E-If1",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::If.into(),
                    Keyword::True.into(),
                    Symbol::brack(Symbol::sub(Symbol::Term, 1)),
                    Keyword::Else.into(),
                    Symbol::brack(Symbol::sub(Symbol::Term, 2)),
                ],
                Symbol::sub(Symbol::Term, 1),
                "E-IfTrue",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::If.into(),
                    Keyword::False.into(),
                    Symbol::brack(Symbol::sub(Symbol::Term, 1)),
                    Keyword::Else.into(),
                    Symbol::brack(Symbol::sub(Symbol::Term, 2)),
                ],
                Symbol::sub(Symbol::Type, 2),
                "E-IfFalse",
            ),
        ])
    }
}
