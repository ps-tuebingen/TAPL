use super::{SolveConstraint, SolveState};
use crate::constraints::{EqualityConstraint, SubtypeConstraint};
use errors::{IndexOutOfBounds, TypeMismatch, inference_error::InferenceError};
use syntax::{
    language::Language,
    span::Spanned,
    types::{Tuple, TypeGroup},
};

impl<Lang> SolveConstraint for Tuple<Lang>
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
        let err = TypeMismatch::new(rhs.to_string(), "Tuple Type".to_string(), rhs.span());
        let rhs_tuple = rhs.into_tuple().ok_or(err)?;
        let self_len = self.tys.len();
        let rhs_len = rhs_tuple.tys.len();
        if self_len != rhs_len {
            let min_len = self_len.min(rhs_len);
            let max_len = self_len.max(rhs_len);
            return Err(IndexOutOfBounds::new(max_len, min_len, self.span).into());
        }

        for (self_ty, rhs_ty) in self.tys.into_iter().zip(rhs_tuple.tys.into_iter()) {
            state.add_constraint(EqualityConstraint::new(self_ty, rhs_ty));
        }
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

        let err = TypeMismatch::new(sup.to_string(), "Tuple Type".to_string(), sup.span());
        let sup_tuple = sup.into_tuple().ok_or(err)?;

        let self_len = self.tys.len();
        let sup_len = sup_tuple.tys.len();

        if self_len > sup_len {
            return Err(IndexOutOfBounds::new(self_len, sup_len, sup_tuple.span).into());
        }

        for (self_ty, sup_ty) in self.tys.into_iter().zip(sup_tuple.tys.into_iter()) {
            state.add_constraint(SubtypeConstraint::new(self_ty, sup_ty));
        }
        Ok(())
    }
}
