use super::{SolveConstraint, SolveState};
use crate::constraints::{EqualityConstraint, SubtypeConstraint};
use errors::{TypeMismatch, inference_error::InferenceError};
use std::rc::Rc;
use syntax::{
    language::Language,
    span::Spanned,
    types::{Sum, TypeGroup},
};

impl<Lang> SolveConstraint for Sum<Lang>
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
        let err = TypeMismatch::new(rhs.to_string(), "Sum Type".to_string(), rhs.span());
        let rhs_sum = rhs.into_sum().ok_or(err)?;
        state.add_constraint(EqualityConstraint::new(
            Rc::unwrap_or_clone(self.left),
            Rc::unwrap_or_clone(rhs_sum.left),
        ));
        state.add_constraint(EqualityConstraint::new(
            Rc::unwrap_or_clone(self.right),
            Rc::unwrap_or_clone(rhs_sum.right),
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

        let err = TypeMismatch::new(sup.to_string(), "Sum Type".to_string(), sup.span());
        let sup_sum = sup.into_sum().ok_or(err)?;
        state.add_constraint(SubtypeConstraint::new(
            Rc::unwrap_or_clone(self.left),
            Rc::unwrap_or_clone(sup_sum.left),
        ));
        state.add_constraint(SubtypeConstraint::new(
            Rc::unwrap_or_clone(self.right),
            Rc::unwrap_or_clone(sup_sum.right),
        ));
        Ok(())
    }
}
