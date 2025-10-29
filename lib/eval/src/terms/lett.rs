use crate::Eval;
use errors::eval_error::EvalError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, SpecialChar, Symbol},
};
use std::{collections::HashSet, rc::Rc};
use syntax::{
    eval_context::EvalContext,
    language::Language,
    subst::SubstTerm,
    terms::{Let, Term},
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Let<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang> + From<Lang::Value>,
    Let<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let bound_res = self.bound_term.eval(env)?;
        let bound_val = bound_res.val();

        let term_subst = self
            .in_term
            .clone()
            .subst(&self.var, &bound_val.clone().into());
        let subst_step = EvalStep::lett(
            Let::new(
                &self.var,
                bound_val,
                Rc::unwrap_or_clone(self.in_term.clone()),
            ),
            Rc::unwrap_or_clone(term_subst.clone()),
        );

        let mut steps = bound_res.congruence(&move |t| {
            Let::new(&self.var, t, Rc::unwrap_or_clone(self.in_term.clone())).into()
        });
        steps.push(subst_step);
        let term_res = term_subst.eval(env)?;
        let val = term_res.val();
        steps.extend(term_res.steps);
        Ok(EvalTrace::<Lang>::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::Let.into(),
                        Symbol::Variable,
                        SpecialChar::Equals.into(),
                        sym,
                        Keyword::In.into(),
                        Symbol::sub(Symbol::Term, 3),
                    ]
                },
                "E-Let1",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Let.into(),
                    Symbol::Variable,
                    SpecialChar::Equals.into(),
                    Symbol::Value,
                    Keyword::In.into(),
                    Symbol::Term,
                ],
                vec![
                    Symbol::Term,
                    SpecialChar::SqBrackO.into(),
                    Symbol::Variable,
                    SpecialChar::Arrow.into(),
                    Symbol::Value,
                    SpecialChar::SqBrackC.into(),
                ],
                "E-Let",
            ),
        ])
    }
}
