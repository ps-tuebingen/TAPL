use crate::Eval;
use errors::eval_error::EvalError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Pack, Term},
    values::Pack as PackVal,
};
use trace::EvalTrace;

impl<Lang> Eval for Pack<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Self: Into<Lang::Term>,
    PackVal<Lang>: Into<Lang::Value>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();

        let val = PackVal::<Lang>::new(self.inner_ty.clone(), term_val, self.outer_ty.clone());

        let steps = term_res.congruence(&move |t| {
            Self::new(self.inner_ty.clone(), t, self.outer_ty.clone()).into()
        });

        Ok(EvalTrace::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
