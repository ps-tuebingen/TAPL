use super::{SolveConstraint, SolveState};
use crate::constraints::{EqualityConstraint, SubtypeConstraint};
use errors::{TypeMismatch, inference_error::InferenceError};
use std::rc::Rc;
use syntax::{
    language::Language,
    span::Spanned,
    types::{Sink, TypeGroup},
};

impl<Lang> SolveConstraint for Sink<Lang>
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
        let err = TypeMismatch::new(rhs.to_string(), "Sink Type".to_string(), rhs.span());
        let rhs_sink = rhs.into_sink().ok_or(err)?;
        state.add_constraint(EqualityConstraint::new(
            Rc::unwrap_or_clone(self.ty),
            Rc::unwrap_or_clone(rhs_sink.ty),
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

        let err = TypeMismatch::new(sup.to_string(), "Sink Type".to_string(), sup.span());
        let sup_sink = sup.into_sink().ok_or(err)?;
        state.add_constraint(SubtypeConstraint::new(
            Rc::unwrap_or_clone(sup_sink.ty),
            Rc::unwrap_or_clone(self.ty),
        ));
        Ok(())
    }
}
