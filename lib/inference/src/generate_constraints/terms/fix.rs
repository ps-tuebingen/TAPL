use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::Fix,
    types::{Fun, TypeVariable},
};

impl<Lang> GenerateConstraints for Fix<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Fun<Lang>: Into<Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;
    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);
        let fun_var = state.fresh_type_var();
        let var_ty = TypeVariable::new(&fun_var, self.span);
        let fun_ty = Fun::new(var_ty.clone(), var_ty.clone(), self.span);
        state.add_constraint(EqualityConstraint::new(fun_ty.clone(), term_ty));
        var_ty.into()
    }
}
