use crate::Eval;
use errors::{IndexOutOfBounds, eval_error::EvalError};
use grammar::{
    DerivationRule,
    symbols::{SpecialChar, Symbol},
};
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Projection, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Projection<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Projection<Lang>: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let tup_val = term_val.clone().into_tuple()?;
        let val = tup_val
            .vals
            .get(self.index)
            .cloned()
            .ok_or(IndexOutOfBounds::new(self.index, tup_val.vals.len()))?;

        let mut steps = term_res.congruence(&move |t| Projection::new(t, self.index).into());
        let last_step = EvalStep::projection(Projection::new(term_val, self.index), val.clone());
        steps.push(last_step);

        Ok(EvalTrace::<Lang>::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| vec![sym, SpecialChar::Dot.into(), SpecialChar::Number.into()],
                "E-Proj1",
            ),
            DerivationRule::eval(
                vec![
                    Symbol::paren(Symbol::many(Symbol::sub(Symbol::Value, "i"))),
                    SpecialChar::Dot.into(),
                    "k".into(),
                ],
                Symbol::sub(Symbol::Value, "k"),
                "E-Proj",
            ),
        ])
    }
}
