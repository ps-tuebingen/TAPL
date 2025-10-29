use crate::Eval;
use errors::eval_error::EvalError;
use grammar::{
    DerivationRule,
    symbols::{Keyword, Symbol},
};
use std::collections::HashSet;
use syntax::{eval_context::EvalContext, language::Language, terms::Cast};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Cast<Lang>
where
    Lang: Language,
    Lang::Term: Eval<Lang = Lang>,
    Cast<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let inner_res = self.term.eval(env)?;
        let inner_val = inner_res.val();
        let last_step = EvalStep::cast(self.ty.clone(), inner_val.clone());
        let mut steps = inner_res.congruence(&move |t| Cast::new(t, self.ty.clone()).into());
        steps.push(last_step);
        Ok(EvalTrace::<Lang>::new(steps, inner_val))
    }
    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(|sym| vec![sym, Keyword::As.into(), Symbol::Type], "E-Cast1"),
            DerivationRule::eval(
                vec![Symbol::Value, Keyword::As.into(), Symbol::Type],
                vec![Symbol::Value],
                "E-Cast",
            ),
        ])
    }
}
