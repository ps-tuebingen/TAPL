use super::{SolveConstraint, SolveState};
use crate::constraints::{EqualityConstraint, SubtypeConstraint};
use errors::{TypeMismatch, inference_error::InferenceError};
use std::rc::Rc;
use syntax::{
    language::Language,
    span::Spanned,
    types::{Source, TypeGroup},
};

impl<Lang> SolveConstraint for Source<Lang>
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
        let err = TypeMismatch::new(rhs.to_string(), "Source Type".to_string(), rhs.span());
        let rhs_source = rhs.into_source().ok_or(err)?;
        state.add_constraint(EqualityConstraint::new(
            Rc::unwrap_or_clone(self.ty),
            Rc::unwrap_or_clone(rhs_source.ty),
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

        let err = TypeMismatch::new(sup.to_string(), "Source Type".to_string(), sup.span());
        let sup_source = sup.into_source().ok_or(err)?;
        state.add_constraint(SubtypeConstraint::new(
            Rc::unwrap_or_clone(self.ty),
            Rc::unwrap_or_clone(sup_source.ty),
        ));
        Ok(())
    }
}
