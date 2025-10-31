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
    terms::{SomeCase, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for SomeCase<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang> + From<Lang::Value>,
    SomeCase<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let bound_res = self.bound_term.eval(env)?;
        let bound_val = bound_res.val();

        let (res_steps, res_val) = if let Ok(some_val) = bound_val.clone().into_something() {
            let some_subst = self
                .some_term
                .clone()
                .subst(&self.some_var, &((*some_val.val).into()));
            let next_step = EvalStep::somecase_some(
                SomeCase::new(
                    bound_val,
                    Rc::unwrap_or_clone(self.none_term.clone()),
                    &self.some_var,
                    Rc::unwrap_or_clone(self.some_term.clone()),
                ),
                Rc::unwrap_or_clone(some_subst.clone()),
            );
            let some_res = some_subst.eval(env)?;
            let some_val = some_res.val();
            let mut some_steps = some_res.steps;
            some_steps.insert(0, next_step);
            (some_steps, some_val)
        } else if bound_val.clone().into_nothing().is_ok() {
            let next_step = EvalStep::somecase_none(
                SomeCase::new(
                    bound_val,
                    Rc::unwrap_or_clone(self.none_term.clone()),
                    &self.some_var,
                    Rc::unwrap_or_clone(self.some_term.clone()),
                ),
                Rc::unwrap_or_clone(self.none_term.clone()),
            );
            let none_res = self.none_term.clone().eval(env)?;
            let none_val = none_res.val();
            let mut none_steps = none_res.steps;
            none_steps.insert(0, next_step);
            (none_steps, none_val)
        } else {
            return Err(ValueMismatch::new(bound_val.to_string(), "Option Term".to_owned()).into());
        };

        let mut steps = bound_res.congruence(&move |t| {
            SomeCase::new(
                t,
                Rc::unwrap_or_clone(self.none_term.clone()),
                &self.some_var,
                Rc::unwrap_or_clone(self.some_term.clone()),
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
                            Keyword::Nothing.into(),
                            SpecialChar::DoubleArrow.into(),
                            Symbol::sub(Symbol::Term, 3),
                            SpecialChar::Pipe.into(),
                            Keyword::Something.into(),
                            Symbol::paren(Symbol::comma_sep(
                                Symbol::sub(Symbol::Variable, 1),
                                Symbol::sub(Symbol::Variable, 2),
                            )),
                            SpecialChar::DoubleArrow.into(),
                            Symbol::sub(Symbol::Term, 4),
                        ]),
                    ]
                },
                "E-SomeCase1",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Case.into(),
                    Keyword::Nothing.into(),
                    Symbol::brack(Symbol::Type),
                    Keyword::Of.into(),
                    Symbol::brack(vec![
                        Keyword::Nothing.into(),
                        SpecialChar::DoubleArrow.into(),
                        Symbol::sub(Symbol::Term, 1),
                        SpecialChar::Pipe.into(),
                        Keyword::Something.into(),
                        Symbol::paren(Symbol::comma_sep(
                            Symbol::sub(Symbol::Variable, 1),
                            Symbol::sub(Symbol::Variable, 2),
                        )),
                        SpecialChar::DoubleArrow.into(),
                        Symbol::sub(Symbol::Term, 2),
                    ]),
                ],
                Symbol::sub(Symbol::Term, 1),
                "E-SomeCaseNothing",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Case.into(),
                    Keyword::Something.into(),
                    Symbol::brack(Symbol::Type),
                    Symbol::paren(Symbol::comma_sep(
                        Symbol::sub(Symbol::Value, 1),
                        Symbol::sub(Symbol::Value, 2),
                    )),
                    Keyword::Of.into(),
                    Symbol::brack(vec![
                        Keyword::Nothing.into(),
                        SpecialChar::DoubleArrow.into(),
                        Symbol::sub(Symbol::Term, 1),
                        SpecialChar::Pipe.into(),
                        Keyword::Something.into(),
                        Symbol::paren(Symbol::comma_sep(
                            Symbol::sub(Symbol::Variable, 1),
                            Symbol::sub(Symbol::Variable, 2),
                        )),
                        SpecialChar::DoubleArrow.into(),
                        Symbol::sub(Symbol::Term, 2),
                    ]),
                ],
                vec![
                    Symbol::sub(Symbol::Term, 2),
                    Symbol::brack(Symbol::comma_sep(
                        Symbol::mapto(
                            Symbol::sub(Symbol::Variable, 1),
                            Symbol::sub(Symbol::Value, 1),
                        ),
                        Symbol::mapto(
                            Symbol::sub(Symbol::Variable, 2),
                            Symbol::sub(Symbol::Value, 2),
                        ),
                    )),
                ],
                "E-SomeCaseSomething",
            ),
        ])
    }
}
