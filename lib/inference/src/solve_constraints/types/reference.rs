use super::{SolveConstraint, SolveState};
use crate::constraints::{EqualityConstraint, SubtypeConstraint};
use errors::{TypeMismatch, inference_error::InferenceError};
use std::rc::Rc;
use syntax::{
    language::Language,
    span::Spanned,
    types::{Reference, TypeGroup},
};

impl<Lang> SolveConstraint for Reference<Lang>
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
        let err = TypeMismatch::new(rhs.to_string(), "Reference".to_string(), rhs.span());
        let rhs_ref = rhs.into_ref().ok_or(err)?;
        state.add_constraint(EqualityConstraint::new(
            Rc::unwrap_or_clone(self.ty),
            Rc::unwrap_or_clone(rhs_ref.ty),
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
        let sup_ref = sup.clone().into_ref();
        if let Some(rf) = sup_ref {
            state.add_constraint(SubtypeConstraint::new(
                Rc::unwrap_or_clone(self.ty),
                Rc::unwrap_or_clone(rf.ty),
            ));
            return Ok(());
        }

        let sup_source = sup.clone().into_source();
        if let Some(src) = sup_source {
            state.add_constraint(SubtypeConstraint::new(
                Rc::unwrap_or_clone(self.ty),
                Rc::unwrap_or_clone(src.ty),
            ));
            return Ok(());
        }

        let sup_sink = sup.clone().into_sink();
        if let Some(snk) = sup_sink {
            state.add_constraint(SubtypeConstraint::new(
                Rc::unwrap_or_clone(snk.ty),
                Rc::unwrap_or_clone(self.ty),
            ));
            return Ok(());
        }

        Err(TypeMismatch::new(self.to_string(), sup.to_string(), self.span).into())
    }
}
