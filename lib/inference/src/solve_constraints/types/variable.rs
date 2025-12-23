use super::{SolveConstraint, SolveState};
use crate::constraints::{EqualityConstraint, SubtypeConstraint};
use errors::inference_error::InferenceError;
use syntax::{language::Language, types::TypeVariable};

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
        };
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
}
