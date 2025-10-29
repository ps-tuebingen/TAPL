use crate::Eval;
use errors::{ValueMismatch, eval_error::EvalError};
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::{collections::HashSet, rc::Rc};
use syntax::{
    eval_context::EvalContext, language::Language, subst::SubstTerm, terms::ListCase,
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for ListCase<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang> + From<Lang::Value>,
    ListCase<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let bound_res = self.bound_term.eval(env)?;
        let bound_val = bound_res.val();

        let (res_steps, val) = if bound_val.clone().into_nil().is_ok() {
            let next_step = EvalStep::listcase_nil(
                ListCase::new(
                    bound_val,
                    Rc::unwrap_or_clone(self.nil_rhs.clone()),
                    &self.cons_fst,
                    &self.cons_rst,
                    Rc::unwrap_or_clone(self.cons_rhs.clone()),
                ),
                Rc::unwrap_or_clone(self.nil_rhs.clone()),
            );
            let nil_res = self.nil_rhs.clone().eval(env)?;
            let nil_val = nil_res.val();
            let mut steps = nil_res.steps;
            steps.insert(0, next_step);
            (steps, nil_val)
        } else if let Ok(cons) = bound_val.clone().into_cons() {
            let cons_subst = self
                .cons_rhs
                .clone()
                .subst(&self.cons_fst, &((*cons.head).into()))
                .subst(&self.cons_rst, &((*cons.tail).into()));
            let next_step = EvalStep::listcase_cons(
                ListCase::new(
                    bound_val,
                    Rc::unwrap_or_clone(self.nil_rhs.clone()),
                    &self.cons_fst,
                    &self.cons_rst,
                    Rc::unwrap_or_clone(self.cons_rhs.clone()),
                ),
                Rc::unwrap_or_clone(cons_subst.clone()),
            );
            let cons_res = cons_subst.eval(env)?;
            let cons_val = cons_res.val();
            let mut steps = cons_res.steps;
            steps.insert(0, next_step);
            (steps, cons_val)
        } else {
            return Err(ValueMismatch::new(bound_val.to_string(), "List".to_owned()).into());
        };

        let mut steps = bound_res.congruence(&move |t| {
            ListCase::new(
                t,
                Rc::unwrap_or_clone(self.nil_rhs.clone()),
                &self.cons_fst,
                &self.cons_rst,
                Rc::unwrap_or_clone(self.cons_rhs.clone()),
            )
            .into()
        });
        steps.extend(res_steps);
        Ok(EvalTrace::<Lang>::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::Case.into(),
                        sym,
                        Keyword::Of.into(),
                        SpecialChar::BrackO.into(),
                        Keyword::Nil.into(),
                        SpecialChar::DoubleArrow.into(),
                        Symbol::sub(Symbol::Term, 3),
                        SpecialChar::Pipe.into(),
                        Keyword::Cons.into(),
                        SpecialChar::ParenO.into(),
                        Symbol::sub(Symbol::Variable, 1),
                        SpecialChar::Comma.into(),
                        Symbol::sub(Symbol::Variable, 2),
                        SpecialChar::ParenC.into(),
                        SpecialChar::DoubleArrow.into(),
                        Symbol::sub(Symbol::Term, 4),
                        SpecialChar::BrackC.into(),
                    ]
                },
                "E-ListCase1",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Case.into(),
                    Keyword::Nil.into(),
                    Keyword::Of.into(),
                    SpecialChar::BrackO.into(),
                    Keyword::Nil.into(),
                    SpecialChar::DoubleArrow.into(),
                    Symbol::sub(Symbol::Term, 3),
                    SpecialChar::Pipe.into(),
                    Keyword::Cons.into(),
                    SpecialChar::ParenO.into(),
                    Symbol::sub(Symbol::Variable, 1),
                    SpecialChar::Comma.into(),
                    Symbol::sub(Symbol::Variable, 2),
                    SpecialChar::ParenC.into(),
                    SpecialChar::DoubleArrow.into(),
                    Symbol::sub(Symbol::Term, 4),
                    SpecialChar::BrackC.into(),
                ],
                Symbol::sub(Symbol::Term, 3),
                "E-ListCaseNil",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Case.into(),
                    Keyword::Cons.into(),
                    SpecialChar::ParenO.into(),
                    Symbol::sub(Symbol::Value, 1),
                    SpecialChar::Comma.into(),
                    Symbol::sub(Symbol::Value, 2),
                    SpecialChar::ParenC.into(),
                    Keyword::Of.into(),
                    SpecialChar::BrackO.into(),
                    Keyword::Nil.into(),
                    SpecialChar::DoubleArrow.into(),
                    Symbol::sub(Symbol::Term, 3),
                    SpecialChar::Pipe.into(),
                    Keyword::Cons.into(),
                    SpecialChar::ParenO.into(),
                    Symbol::sub(Symbol::Variable, 1),
                    SpecialChar::Comma.into(),
                    Symbol::sub(Symbol::Variable, 2),
                    SpecialChar::ParenC.into(),
                    SpecialChar::DoubleArrow.into(),
                    Symbol::sub(Symbol::Term, 4),
                    SpecialChar::BrackC.into(),
                ],
                vec![
                    Symbol::sub(Symbol::Term, 4),
                    SpecialChar::SqBrackO.into(),
                    Symbol::sub(Symbol::Variable, 1),
                    SpecialChar::Arrow.into(),
                    Symbol::sub(Symbol::Value, 1),
                    SpecialChar::Arrow.into(),
                    Symbol::sub(Symbol::Variable, 2),
                    SpecialChar::Arrow.into(),
                    Symbol::sub(Symbol::Value, 2),
                    SpecialChar::SqBrackC.into(),
                ],
                "E-ListCaseCons",
            ),
        ])
    }
}
