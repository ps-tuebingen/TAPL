use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::Assign,
    types::{Reference, TypeVariable, Unit},
};

impl<Lang> GenerateConstraints for Assign<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
    Reference<Lang>: Into<Lang::Type>,
    Unit<Lang>: Into<Lang::Type>,
{
    type Target = Lang::Type;
    type Lang = Lang;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let lhs_ty = self.lhs.generate_constraints(state);
        let ref_var = state.fresh_type_var();
        let var_ty = TypeVariable::new(&ref_var, self.span);
        let lhs_ref = Reference::<Lang>::new(var_ty.clone(), self.span);
        state.add_constraint(EqualityConstraint::new(lhs_ty, lhs_ref));

        let rhs_ty = self.rhs.generate_constraints(state);
        state.add_constraint(EqualityConstraint::new(rhs_ty, var_ty));
        Unit::new(self.span).into()
    }
}
