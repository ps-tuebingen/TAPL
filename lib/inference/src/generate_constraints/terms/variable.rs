use super::{GenState, GenerateConstraints};
use syntax::{language::Language, terms::Variable, types::TypeVariable};

impl<Lang> GenerateConstraints for Variable<Lang>
where
    Lang: Language,
    TypeVariable<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        if let Some(ty) = state.var_types.get(&self.var) {
            return ty.clone();
        }
        let ty_var = state.fresh_type_var();
        TypeVariable::new(&ty_var, self.span).into()
    }
}
