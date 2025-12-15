use crate::Eval;
use errors::{UndefinedLabel, ValueMismatch, eval_error::EvalError};
use grammar::{
    DerivationRule,
    symbols::{SpecialChar, Symbol},
};
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{RecordProj, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for RecordProj<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.record.eval(env)?;
        let term_val = term_res.val();
        let rec_val = term_val
            .clone()
            .into_record()
            .ok_or_else(|| ValueMismatch::new(term_val.to_string(), "Record Value".to_string()))?;
        let val = rec_val
            .records
            .get(&self.label)
            .cloned()
            .ok_or_else(|| UndefinedLabel::new(&self.label))?;

        let last_step =
            EvalStep::recordproj(Self::new(val.clone(), &self.label, self.span), val.clone());

        let mut steps = term_res.congruence(&move |t| Self::new(t, &self.label, self.span).into());
        steps.push(last_step);
        Ok(EvalTrace::<Lang>::new(steps, val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval_cong(
                |sym| vec![sym, SpecialChar::Dot.into(), Symbol::Label],
                "E-RecordProj1",
            ),
            DerivationRule::eval(
                vec![
                    Symbol::brack(vec![Symbol::many(vec![
                        Symbol::sub(Symbol::Label, "i"),
                        SpecialChar::Equals.into(),
                        Symbol::sub(Symbol::Value, "i"),
                    ])]),
                    SpecialChar::Dot.into(),
                    Symbol::sub(Symbol::Label, "k"),
                ],
                Symbol::sub(Symbol::Value, "k"),
                "E-RecordProj",
            ),
        ])
    }
}
