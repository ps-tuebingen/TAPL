use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::Tail,
    types::{List, TypeVariable},
};

impl<Lang> GenerateConstraints for Tail<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
    List<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);
        let list_var = state.fresh_type_var();
        let ty_var = TypeVariable::new(&list_var, self.span);
        let list_ty = List::new(ty_var, self.span);

        state.add_constraint(EqualityConstraint::new(list_ty.clone(), term_ty));
        list_ty.into()
    }
}
