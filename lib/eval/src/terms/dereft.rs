use crate::Eval;
use errors::{UndefinedLocation, ValueMismatch, eval_error::EvalError};
use grammar::{
    DerivationRule,
    symbols::{SpecialChar, Symbol},
};
use std::collections::HashSet;
use syntax::{
    eval_context::EvalContext,
    language::Language,
    terms::{Deref, Term},
    values::ValueGroup,
};
use trace::{EvalStep, EvalTrace};

impl<Lang> Eval for Deref<Lang>
where
    Lang: Language,
    Lang::Term: Term + Eval<Lang = Lang>,
    Self: Into<Lang::Term>,
{
    type Lang = Lang;

    fn eval(self, env: &mut EvalContext<Lang>) -> Result<EvalTrace<Lang>, EvalError> {
        let term_res = self.term.eval(env)?;
        let term_val = term_res.val();
        let loc_val = term_val.clone().into_loc().ok_or(ValueMismatch::new(
            term_val.to_string(),
            "Location Value".to_string(),
        ))?;

        let loc_val = env
            .get_location(loc_val.loc)
            .ok_or(UndefinedLocation::new(loc_val.loc))?;
        let last_step = EvalStep::deref(term_val, loc_val.clone(), self.span);
        let mut steps = term_res.congruence(&move |t| Self::new(t, self.span).into());
        steps.push(last_step);
        Ok(EvalTrace::<Lang>::new(steps, loc_val))
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::from([
            DerivationRule::eval(
                vec![
                    SpecialChar::Exclamation.into(),
                    Symbol::Location,
                    SpecialChar::Pipe.into(),
                    SpecialChar::Mu.into(),
                ],
                vec![
                    SpecialChar::Mu.into(),
                    Symbol::paren(Symbol::Location),
                    SpecialChar::Pipe.into(),
                    SpecialChar::Mu.into(),
                ],
                "E-Deref",
            ),
            DerivationRule::eval_cong(
                |sym| vec![SpecialChar::Exclamation.into(), sym, SpecialChar::Mu.into()],
                "E-Deref1",
            ),
        ])
    }
}
