use super::{EqualityConstraint, GenState, GenerateConstraints, SubtypeConstraint};
use syntax::{
    language::Language,
    terms::App,
    types::{Fun, TypeVariable},
};

impl<Lang> GenerateConstraints for App<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Fun<Lang>: Into<Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;
    fn generate_constraints(&self, state: &mut GenState<Self::Lang>) -> Self::Target {
        let features = Lang::features();

        let fun_ty = self.fun.generate_constraints(state);
        let arg_ty = self.arg.generate_constraints(state);
        let to_var = state.fresh_type_var();
        let to_ty = TypeVariable::new(&to_var, self.span);
        let ty_var = state.fresh_type_var();
        let from_var = TypeVariable::new(&ty_var, self.span);
        let fun_vars = Fun::new(from_var.clone(), to_ty.clone(), self.span);

        state.add_constraint(EqualityConstraint::new(fun_vars, fun_ty));

        if features.subtyped() {
            state.add_constraint(SubtypeConstraint::new(from_var, arg_ty));
        } else {
            state.add_constraint(EqualityConstraint::new(from_var, arg_ty));
        }

        to_ty.into()
    }
}
