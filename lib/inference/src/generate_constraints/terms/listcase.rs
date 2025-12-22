use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::ListCase,
    types::{List, TypeVariable},
};

impl<Lang> GenerateConstraints for ListCase<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
    List<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;
    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let bound_ty = self.bound_term.generate_constraints(state);
        let list_var = state.fresh_type_var();
        let var_ty = TypeVariable::new(&list_var, self.span);
        let list_ty = List::new(var_ty.clone(), self.span);

        state.add_constraint(EqualityConstraint::new(bound_ty, list_ty.clone()));
        let nil_ty = self.nil_rhs.generate_constraints(state);
        state.add_var(&self.cons_fst, var_ty);
        state.add_var(&self.cons_rst, list_ty);
        let cons_ty = self.cons_rhs.generate_constraints(state);
        state.add_constraint(EqualityConstraint::new(nil_ty.clone(), cons_ty));
        nil_ty
    }
}
