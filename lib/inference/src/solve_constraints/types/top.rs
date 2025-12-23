use super::{SolveConstraint, SolveState};
use errors::{TypeMismatch, inference_error::InferenceError};
use syntax::{
    language::Language,
    span::Spanned,
    types::{Top, TypeGroup},
};

impl<Lang> SolveConstraint for Top<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    fn solve_equality(
        self,
        rhs: Lang::Type,
        _: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        let err = TypeMismatch::new(rhs.to_string(), "Top".to_string(), rhs.span());
        rhs.into_top().map(|_| ()).ok_or(err.into())
    }

    fn solve_subtyping(
        self,
        sup: Lang::Type,
        _: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        let err = TypeMismatch::new(sup.to_string(), "Top".to_string(), sup.span());
        sup.into_top().map(|_| ()).ok_or(err.into())
    }
}
