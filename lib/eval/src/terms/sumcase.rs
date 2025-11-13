use crate::Eval;
use errors::{ValueMismatch, eval_error::EvalError};
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::{collections::HashSet, rc::Rc};
use syntax::{
    eval_context::EvalContext,
    language::Language,
    subst::SubstTerm,
    terms::{SumCase, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for SumCase<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang> + From<Lang::Value>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let bound_res = self.bound_term.eval(env)?;
        let bound_val = bound_res.val();

        let (res_steps, res_val) = if let Ok(left_val) = bound_val.clone().into_left() {
            let left_subst = self
                .left_term
                .clone()
                .subst(&self.left_var, &((*left_val.left_val).into()));
            let next_step = EvalStep::sumcase_left(
                Self::new(
                    bound_val,
                    &self.left_var,
                    Rc::unwrap_or_clone(self.left_term.clone()),
                    &self.right_var,
                    Rc::unwrap_or_clone(self.right_term.clone()),
                ),
                Rc::unwrap_or_clone(left_subst.clone()),
            );
            let left_res = left_subst.eval(env)?;
            let left_var = left_res.val();
            let mut left_steps = left_res.steps;
            left_steps.insert(0, next_step);
            (left_steps, left_var)
        } else if let Ok(right_val) = bound_val.clone().into_right() {
            let right_subst = self
                .right_term
                .clone()
                .subst(&self.right_var, &((*right_val.right_val).into()));
            let next_step = EvalStep::sumcase_right(
                Self::new(
                    bound_val,
                    &self.left_var,
                    Rc::unwrap_or_clone(self.left_term.clone()),
                    &self.right_var,
                    Rc::unwrap_or_clone(self.right_term.clone()),
                ),
                Rc::unwrap_or_clone(right_subst.clone()),
            );
            let right_res = right_subst.eval(env)?;
            let right_val = right_res.val();
            let mut right_steps = right_res.steps;
            right_steps.insert(0, next_step);
            (right_steps, right_val)
        } else {
            return Err(ValueMismatch::new(bound_val.to_string(), "Sum Term".to_owned()).into());
        };

        let mut steps = bound_res.congruence(&move |t| {
            Self::new(
                t,
                &self.left_var,
                Rc::unwrap_or_clone(self.left_term.clone()),
                &self.right_var,
                Rc::unwrap_or_clone(self.right_term.clone()),
            )
            .into()
        });
        steps.extend(res_steps);
        Ok(EvalTrace::<Lang>::new(steps, res_val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::Case.into(),
                        sym,
                        Keyword::Of.into(),
                        Symbol::brack(vec![
                            Keyword::Left.into(),
                            Symbol::paren(Symbol::sub(Symbol::Variable, 1)),
                            SpecialChar::DoubleArrow.into(),
                            Symbol::sub(Symbol::Term, 1),
                            SpecialChar::Pipe.into(),
                            Keyword::Right.into(),
                            Symbol::paren(Symbol::sub(Symbol::Variable, 2)),
                            SpecialChar::DoubleArrow.into(),
                            Symbol::sub(Symbol::Term, 2),
                        ]),
                    ]
                },
                "E-SumCase1",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Case.into(),
                    Keyword::Left.into(),
                    Symbol::brack(Symbol::Type),
                    Symbol::paren(Symbol::sub(Symbol::Term, 4)),
                    Keyword::Of.into(),
                    Symbol::brack(vec![
                        Keyword::Left.into(),
                        Symbol::paren(Symbol::sub(Symbol::Variable, 1)),
                        SpecialChar::DoubleArrow.into(),
                        Symbol::sub(Symbol::Term, 1),
                        SpecialChar::Pipe.into(),
                        Keyword::Right.into(),
                        Symbol::paren(Symbol::sub(Symbol::Variable, 2)),
                        SpecialChar::DoubleArrow.into(),
                        Symbol::sub(Symbol::Term, 2),
                    ]),
                ],
                vec![
                    Symbol::sub(Symbol::Term, 1),
                    Symbol::sqbrack(Symbol::mapto(
                        Symbol::sub(Symbol::Variable, 1),
                        Symbol::sub(Symbol::Term, 3),
                    )),
                ],
                "E-SumCaseLeft",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Case.into(),
                    Keyword::Right.into(),
                    Symbol::brack(Symbol::Type),
                    Symbol::paren(Symbol::sub(Symbol::Term, 4)),
                    Keyword::Of.into(),
                    Symbol::brack(vec![
                        Keyword::Left.into(),
                        Symbol::paren(Symbol::sub(Symbol::Variable, 1)),
                        SpecialChar::DoubleArrow.into(),
                        Symbol::sub(Symbol::Term, 1),
                        SpecialChar::Pipe.into(),
                        Keyword::Right.into(),
                        Symbol::paren(Symbol::sub(Symbol::Variable, 2)),
                        SpecialChar::DoubleArrow.into(),
                        Symbol::sub(Symbol::Term, 2),
                    ]),
                ],
                vec![
                    Symbol::sub(Symbol::Term, 2),
                    Symbol::sqbrack(Symbol::mapto(
                        Symbol::sub(Symbol::Variable, 2),
                        Symbol::sub(Symbol::Term, 3),
                    )),
                ],
                "E-SumCaseRight",
            ),
        ])
    }
}
