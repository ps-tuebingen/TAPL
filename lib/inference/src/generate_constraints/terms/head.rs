use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::Head,
    types::{List, TypeVariable},
};

impl<Lang> GenerateConstraints for Head<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    List<Lang>: Into<Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);
        let list_var = state.fresh_type_var();
        let var_ty = TypeVariable::new(&list_var, self.span);
        let list_ty = List::new(var_ty.clone(), self.span);
        state.add_constraint(EqualityConstraint::new(term_ty, list_ty.clone()));
        state.add_constraint(EqualityConstraint::new(list_ty, self.ty.clone()));
        var_ty.into()
    }
}
