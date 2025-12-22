use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::Nil,
    types::{List, TypeVariable},
};

impl<Lang> GenerateConstraints for Nil<Lang>
where
    Lang: Language,
    List<Lang>: Into<Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;
    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let list_var = state.fresh_type_var();
        let var_ty = TypeVariable::new(&list_var, self.span);
        let list_ty = List::new(var_ty, self.span);
        state.add_constraint(EqualityConstraint::new(list_ty.clone(), self.ty.clone()));
        list_ty.into()
    }
}
