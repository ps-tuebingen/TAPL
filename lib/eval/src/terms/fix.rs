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
    subst::SubstTerm,
    terms::{Fix, Term},
    values::ValueGroup,
};
use trace::EvalTrace;

impl<Lang> Eval for Fix<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.clone().eval(env)?;
        let term_val = term_res.val();
        let lam_val = term_val.into_lambda()?;

        let mut steps = term_res.congruence(&move |t| Fix::new(t).into());

        let body_subst = lam_val.body.subst(&lam_val.var, &self.into());
        let body_res = body_subst.eval(env)?;
        let body_val = body_res.val();
        steps.extend(body_res.steps);
        Ok(EvalTrace::<Lang>::new(steps, body_val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| {
                    vec![
                        Keyword::Fix.into(),
                        SpecialChar::ParenO.into(),
                        sym,
                        SpecialChar::ParenC.into(),
                    ]
                },
                "E-Fix1",
            ),
            DerivationRule::eval(
                vec![
                    Keyword::Fix.into(),
                    SpecialChar::ParenO.into(),
                    SpecialChar::Lambda.into(),
                    Symbol::Variable,
                    SpecialChar::Colon.into(),
                    Symbol::Type,
                    SpecialChar::Dot.into(),
                    Symbol::Term,
                    SpecialChar::ParenC.into(),
                ],
                vec![
                    Symbol::Term,
                    SpecialChar::SqBrackO.into(),
                    Symbol::Variable,
                    SpecialChar::Arrow.into(),
                    Keyword::Fix.into(),
                    SpecialChar::Lambda.into(),
                    Symbol::Variable,
                    SpecialChar::Colon.into(),
                    Symbol::Type,
                    SpecialChar::Dot.into(),
                    Symbol::Term,
                    SpecialChar::SqBrackC.into(),
                ],
                "E-Fix",
            ),
        ])
    }
}
