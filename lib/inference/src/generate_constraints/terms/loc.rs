use super::{GenState, GenerateConstraints};
use syntax::{language::Language, terms::Loc, types::TypeVariable};

impl<Lang> GenerateConstraints for Loc<Lang>
where
    Lang: Language,
    TypeVariable<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        match state.loc_types.get(&self.loc) {
            Some(ty) => ty.clone(),
            None => {
                let ty_var = state.fresh_type_var();
                TypeVariable::new(&ty_var, self.span).into()
            }
        }
    }
}
