use crate::Eval;
use errors::eval_error::EvalError;
use grammar::DerivationRule;
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Term, Variable},
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Variable<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, ctx: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let body = ctx.get_name(&self.var)?;
        let mut term_res = body.clone().eval(ctx)?;
        term_res
            .steps
            .insert(0, EvalStep::subst_var(&self.var, body));
        Ok(term_res)
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
    }
}
