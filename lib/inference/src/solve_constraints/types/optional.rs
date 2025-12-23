use super::{SolveConstraint, SolveState};
use crate::constraints::{EqualityConstraint, SubtypeConstraint};
use errors::{TypeMismatch, inference_error::InferenceError};
use std::rc::Rc;
use syntax::{
    language::Language,
    types::{Optional, TypeGroup},
};

impl<Lang> SolveConstraint for Optional<Lang>
where
    Lang: Language,
    Lang::Type: TypeGroup<Lang = Lang>,
{
    type Lang = Lang;
    fn solve_equality(
        self,
        rhs: Lang::Type,
        state: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        let err = TypeMismatch::new(rhs.to_string(), "Option Type".to_string(), self.span);
        let rhs_option = rhs.into_optional().ok_or(err)?;
        state.add_constraint(EqualityConstraint::new(
            Rc::unwrap_or_clone(rhs_option.ty),
            Rc::unwrap_or_clone(self.ty),
        ));
        Ok(())
    }

    fn solve_subtyping(
        self,
        sup: Lang::Type,
        state: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        if sup.clone().into_top().is_some() {
            return Ok(());
        }
        let err = TypeMismatch::new(sup.to_string(), "Option Type".to_string(), self.span);
        let sup_option = sup.into_optional().ok_or(err)?;
        state.add_constraint(SubtypeConstraint::new(
            Rc::unwrap_or_clone(self.ty),
            Rc::unwrap_or_clone(sup_option.ty),
        ));
        Ok(())
    }
}
