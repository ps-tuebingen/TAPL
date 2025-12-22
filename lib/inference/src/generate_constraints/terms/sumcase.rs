use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::SumCase,
    types::{Sum, TypeVariable},
};

impl<Lang> GenerateConstraints for SumCase<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
    Sum<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let bound_ty = self.bound_term.generate_constraints(state);
        let left_var = state.fresh_type_var();
        let left_ty = TypeVariable::new(&left_var, self.span);
        let right_var = state.fresh_type_var();
        let right_ty = TypeVariable::new(&right_var, self.span);
        let sum_ty = Sum::new(left_ty.clone(), right_ty.clone(), self.span);
        state.add_constraint(EqualityConstraint::new(sum_ty, bound_ty));

        state.add_var(&self.left_var, left_ty);
        let left_rhs_ty = self.left_term.generate_constraints(state);
        state.var_types.remove(&self.left_var);

        state.add_var(&self.right_var, right_ty);
        let right_rhs_ty = self.right_term.generate_constraints(state);

        state.add_constraint(EqualityConstraint::new(left_rhs_ty.clone(), right_rhs_ty));
        left_rhs_ty
    }
}
