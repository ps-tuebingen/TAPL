use super::{SolveConstraint, SolveState};
use crate::constraints::{
    EqualityConstraint, IndexConstraint, RecordConstraint, SubtypeConstraint, VariantConstraint,
};
use errors::inference_error::InferenceError;
use syntax::{Label, language::Language, types::TypeVariable};

impl<Lang> SolveConstraint for TypeVariable<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    fn solve_equality(
        self,
        rhs: Lang::Type,
        state: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        match state.var_tys.get(&self.v) {
            None => {
                state.var_tys.insert(self.v.clone(), rhs);
            }
            Some(ty) => {
                state.add_constraint(EqualityConstraint::new(ty.clone(), rhs));
            }
        }
        Ok(())
    }

    fn solve_subtyping(
        self,
        sup: Lang::Type,
        state: &mut SolveState<Lang>,
    ) -> Result<(), InferenceError> {
        match state.tyvar_super.get(&self.v) {
            None => {
                state.tyvar_super.insert(self.v.clone(), sup);
            }
            Some(ty) => state.add_constraint(SubtypeConstraint::new(ty.clone(), sup)),
        }
        Ok(())
    }

    fn solve_index(
        self,
        ind: usize,
        ind_ty: <Self::Lang as Language>::Type,
        state: &mut SolveState<Self::Lang>,
    ) -> Result<(), InferenceError> {
        match state.var_tys.get(&self.v) {
            None => Ok(()),
            Some(ty) => {
                state.add_constraint(IndexConstraint::new(ty.clone(), ind, ind_ty));
                Ok(())
            }
        }
    }

    fn solve_record(
        self,
        lb: Label,
        label_ty: <Self::Lang as Language>::Type,
        state: &mut SolveState<Self::Lang>,
    ) -> Result<(), InferenceError> {
        match state.var_tys.get(&self.v) {
            None => Ok(()),
            Some(ty) => {
                state.add_constraint(RecordConstraint::new(ty.clone(), &lb, label_ty));
                Ok(())
            }
        }
    }

    fn solve_variant(
        self,
        lb: Label,
        label_ty: <Self::Lang as Language>::Type,
        state: &mut SolveState<Self::Lang>,
    ) -> Result<(), InferenceError> {
        match state.var_tys.get(&self.v) {
            None => Ok(()),
            Some(ty) => {
                state.add_constraint(VariantConstraint::new(ty.clone(), &lb, label_ty));
                Ok(())
            }
        }
    }
}
