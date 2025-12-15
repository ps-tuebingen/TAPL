use crate::Eval;
use errors::{IndexOutOfBounds, ValueMismatch, eval_error::EvalError};
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
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let tup_val = term_val.clone().into_tuple().ok_or(ValueMismatch::new(
            term_val.to_string(),
            "Tuple Value".to_string(),
        ))?;
        let val = tup_val
            .vals
            .get(self.index)
            .cloned()
            .ok_or_else(|| IndexOutOfBounds::new(self.index, tup_val.vals.len()))?;

        let mut steps = term_res.congruence(&move |t| Self::new(t, self.index, self.span).into());
        let last_step =
            EvalStep::projection(Self::new(term_val, self.index, self.span), val.clone());
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
