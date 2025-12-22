use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::Nothing,
    types::{Optional, TypeVariable},
};

impl<Lang> GenerateConstraints for Nothing<Lang>
where
    Lang: Language,
    TypeVariable<Lang>: Into<Lang::Type>,
    Optional<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let option_var = state.fresh_type_var();
        let var_ty = TypeVariable::new(&option_var, self.span);
        let option_ty = Optional::new(var_ty, self.span);
        state.add_constraint(EqualityConstraint::new(self.ty.clone(), option_ty.clone()));
        option_ty.into()
    }
}
