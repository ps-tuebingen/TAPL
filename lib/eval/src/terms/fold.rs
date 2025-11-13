use crate::Eval;
use errors::eval_error::EvalError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, Symbol},
};
use std::collections::HashSet;
use syntax::{eval_context::EvalContext, language::Language, terms::Fold, values::Fold as FoldVal};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Fold<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
    Self: Into<Lang::Term>,
    FoldVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let val: <Lang as Language>::Value =
            FoldVal::<Lang>::new(self.ty.clone(), term_val.clone()).into();
        let last_step = EvalStep::fold(Self::new(term_val, self.ty.clone()), val.clone());
        let mut steps = term_res.congruence(&move |t| Self::new(t, self.ty.clone()).into());
        steps.push(last_step);
        Ok(EvalTrace::<Lang>::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([DerivationRule::eval_cong(
            |sym| {
                vec![
                    Keyword::Fold.into(),
                    Symbol::sqbrack(Symbol::Term),
                    Symbol::paren(sym),
                ]
            },
            "E-Fold1",
        )])
    }
}
