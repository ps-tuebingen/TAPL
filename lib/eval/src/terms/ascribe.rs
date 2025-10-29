use crate::Eval;
use errors::eval_error::EvalError;
use grammar::{
    DerivationRule,
    symbols::{SpecialChar, Symbol},
};
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Ascribe, Term},
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Ascribe<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang> + From<Ascribe<Lang>> + From<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let inner_res = self.term.clone().eval(env)?;
        let val = inner_res.val();
        let last_step = EvalStep::ascribe(Ascribe::new(val.clone(), self.ty.clone()), val.clone());
        let mut steps = inner_res.congruence(&move |t| Ascribe::new(t, self.ty.clone()).into());
        steps.push(last_step);
        Ok(EvalTrace::<Lang>::new(steps, val))
    }
    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| vec![sym, SpecialChar::Colon.into(), Symbol::Type],
                "E-Asc1",
            ),
            DerivationRule::eval(
                vec![Symbol::Value, SpecialChar::Colon.into(), Symbol::Type],
                vec![Symbol::Value],
                "E-Asc",
            ),
        ])
    }
}
