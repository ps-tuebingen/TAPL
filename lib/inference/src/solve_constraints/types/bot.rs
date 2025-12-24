use super::{SolveConstraint, SolveState};
use crate::constraints::KindConstraint;
use errors::{TypeMismatch, inference_error::InferenceError};
use syntax::{
    language::Language,
    span::Spanned,
    types::{Bot, TypeGroup},
};

impl<Lang> SolveConstraint for Bot<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    fn solve_equality(
        self,
        rhs: Lang::Type,
        state: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        let err = TypeMismatch::new(rhs.to_string(), "Bottom Type".to_string(), rhs.span());
        let rhs_bot = rhs.into_bot().ok_or(err)?;
        state.add_constraint(KindConstraint::new(rhs_bot.kind, self.kind, self.span));
        Ok(())
    }

    fn solve_subtyping(
        self,
        _: Lang::Type,
        _: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        Ok(())
    }
}
