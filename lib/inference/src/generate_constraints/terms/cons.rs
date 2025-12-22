use super::{EqualityConstraint, GenState, GenerateConstraints, SubtypeConstraint};
use syntax::{
    language::Language,
    terms::Cons,
    types::{List, TypeVariable},
};

impl<Lang> GenerateConstraints for Cons<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
    List<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let features = Lang::features();

        let head_ty = self.head.generate_constraints(state);
        let tail_ty = self.tail.generate_constraints(state);
        let list_var = state.fresh_type_var();
        let var_ty = TypeVariable::new(&list_var, self.span);
        let list_ty = List::new(var_ty.clone(), self.span);

        if features.subtyped() {
            state.add_constraint(SubtypeConstraint::new(tail_ty, list_ty.clone()));
            state.add_constraint(SubtypeConstraint::new(head_ty, var_ty));
        } else {
            state.add_constraint(EqualityConstraint::new(list_ty.clone(), tail_ty));
            state.add_constraint(EqualityConstraint::new(var_ty, head_ty));
        }
        list_ty.into()
    }
}
