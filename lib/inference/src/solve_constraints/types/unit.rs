use super::{SolveConstraint, SolveState};
use errors::{TypeMismatch, inference_error::InferenceError};
use syntax::{
    language::Language,
    span::Spanned,
    types::{TypeGroup, Unit},
};

impl<Lang> SolveConstraint for Unit<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    fn solve_equality(
        self,
        rhs: Lang::Type,
        _: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        let err = TypeMismatch::new(rhs.to_string(), "Unit Type".to_string(), rhs.span());
        rhs.into_unit().ok_or(err)?;
        Ok(())
    }

    fn solve_subtyping(
        self,
        sup: Lang::Type,
        _: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        let err = TypeMismatch::new(self.to_string(), sup.to_string(), self.span);
        if sup.clone().into_top().is_some() || sup.into_unit().is_some() {
            Ok(())
        } else {
            Err(err.into())
        }
    }
}
