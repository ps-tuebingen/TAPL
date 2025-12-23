use super::{SolveConstraint, SolveState};
use crate::constraints::{EqualityConstraint, SubtypeConstraint};
use errors::{TypeMismatch, UndefinedLabel, inference_error::InferenceError};
use syntax::{
    language::Language,
    span::Spanned,
    types::{TypeGroup, Variant},
};

impl<Lang> SolveConstraint for Variant<Lang>
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
        let err = TypeMismatch::new(rhs.to_string(), "Variant Type".to_string(), rhs.span());
        let mut rhs_variant = rhs.into_variant().ok_or(err)?;
        for (lb, ty) in self.variants {
            let err = UndefinedLabel::new(&lb, self.span);
            let rhs_ty = rhs_variant.variants.remove(&lb).ok_or(err)?;
            state.add_constraint(EqualityConstraint::new(ty, rhs_ty));
        }
        if let Some(lb) = rhs_variant.variants.keys().next() {
            return Err(UndefinedLabel::new(&lb, rhs_variant.span).into());
        }
        Ok(())
    }

    fn solve_subtyping(
        mut self,
        sup: Lang::Type,
        state: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        if sup.clone().into_top().is_some() {
            return Ok(());
        }

        let err = TypeMismatch::new(sup.to_string(), "Variant Type".to_string(), sup.span());
        let sup_variant = sup.into_variant().ok_or(err)?;

        for (lb, ty) in sup_variant.variants {
            let err = UndefinedLabel::new(&lb, self.span);
            let self_ty = self.variants.remove(&lb).ok_or(err)?;
            state.add_constraint(SubtypeConstraint::new(self_ty, ty));
        }
        Ok(())
    }
}
