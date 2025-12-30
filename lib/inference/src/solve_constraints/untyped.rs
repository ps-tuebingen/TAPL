use super::{SolveConstraint, SolveState};
use errors::{NoTyping, inference_error::InferenceError};
use syntax::{Label, language::Language, untyped::Untyped};

impl<Lang> SolveConstraint for Untyped<Lang>
where
    Lang: Language,
{
    type Lang = Lang;

    fn solve_equality(
        self,
        _: <Self::Lang as Language>::Type,
        _: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        Err(NoTyping::new(Lang::describe()).into())
    }

    fn solve_subtyping(
        self,
        _: <Self::Lang as Language>::Type,
        _: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        Err(NoTyping::new(Lang::describe()).into())
    }

    fn solve_index(
        self,
        _: usize,
        _: <Self::Lang as Language>::Type,
        _: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        Err(NoTyping::new(Lang::describe()).into())
    }

    fn solve_record(
        self,
        _: Label,
        _: <Self::Lang as Language>::Type,
        _: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        Err(NoTyping::new(Lang::describe()).into())
    }

    fn solve_variant(
        self,
        _: Label,
        _: <Self::Lang as Language>::Type,
        _: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        Err(NoTyping::new(Lang::describe()).into())
    }
}
