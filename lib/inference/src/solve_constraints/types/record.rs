use super::{SolveConstraint, SolveState};
use crate::constraints::{EqualityConstraint, SubtypeConstraint};
use errors::{TypeMismatch, UndefinedLabel, inference_error::InferenceError};
use syntax::{
    Label,
    language::Language,
    span::Spanned,
    types::{Record, TypeGroup},
};

impl<Lang> SolveConstraint for Record<Lang>
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
        let err = TypeMismatch::new(rhs.to_string(), "Record Type".to_string(), rhs.span());
        let mut rhs_record = rhs.into_record().ok_or(err)?;
        for (lb, ty) in self.records {
            let err = UndefinedLabel::new(&lb, rhs_record.span);
            let rhs_ty = rhs_record.records.remove(&lb).ok_or(err)?;
            state.add_constraint(EqualityConstraint::new(ty, rhs_ty));
        }
        if let Some(lb) = rhs_record.records.keys().next() {
            return Err(UndefinedLabel::new(lb, rhs_record.span).into());
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
        let err = TypeMismatch::new(sup.to_string(), "Record Type".to_string(), sup.span());
        let mut sup_record = sup.into_record().ok_or(err)?;
        for (lb, ty) in self.records {
            let err = UndefinedLabel::new(&lb, sup_record.span());
            let sup_ty = sup_record.records.remove(&lb).ok_or(err)?;
            state.add_constraint(SubtypeConstraint::new(ty, sup_ty));
        }
        Ok(())
    }

    fn solve_record(
        mut self,
        lb: Label,
        lb_ty: <Self::Lang as Language>::Type,
        state: &mut SolveState<Self::Lang>,
    ) -> Result<(), InferenceError> {
        let self_ty = self
            .records
            .remove(&lb)
            .ok_or(UndefinedLabel::new(&lb, self.span))?;
        state.add_constraint(EqualityConstraint::new(lb_ty, self_ty));
        Ok(())
    }
}
