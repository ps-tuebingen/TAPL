use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::SomeCase,
    types::{Optional, TypeVariable},
};

impl<Lang> GenerateConstraints for SomeCase<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
    Optional<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let bound_ty = self.bound_term.generate_constraints(state);
        let option_var = state.fresh_type_var();
        let var_ty = TypeVariable::new(&option_var, self.span);
        let some_ty = Optional::new(var_ty.clone(), self.span);

        state.add_constraint(EqualityConstraint::new(bound_ty, some_ty));

        let none_ty = self.none_term.generate_constraints(state);
        state.add_var(&self.some_var, var_ty);
        let some_ty = self.some_term.generate_constraints(state);

        state.add_constraint(EqualityConstraint::new(none_ty, some_ty.clone()));
        some_ty
    }
}
