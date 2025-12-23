use super::{SolveConstraint, SolveState};
use errors::{TypeMismatch, inference_error::InferenceError};
use syntax::{
    language::Language,
    types::{Nat, TypeGroup},
};

impl<Lang> SolveConstraint for Nat<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    fn solve_equality(
        self,
        rhs: Lang::Type,
        _: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        let rhs_str = rhs.to_string();
        if rhs.into_nat().is_some() {
            Ok(())
        } else {
            Err(TypeMismatch::new(self.to_string(), rhs_str, self.span).into())
        }
    }

    fn solve_subtyping(
        self,
        sup: Lang::Type,
        _: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        let sup_str = sup.to_string();
        if sup.clone().into_top().is_some() || sup.into_nat().is_some() {
            Ok(())
        } else {
            Err(TypeMismatch::new(self.to_string(), sup_str, self.span).into())
        }
    }
}
