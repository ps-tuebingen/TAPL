use super::{GenState, GenerateConstraints};
use syntax::{kinds::Kind, language::Language, types::TypeVariable};

impl<Lang> GenerateConstraints for TypeVariable<Lang>
where
    Lang: Language,
    Lang::Type: GenerateConstraints<Lang = Lang, Target = Kind>,
{
    type Lang = Lang;
    type Target = Kind;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        match state.var_types.get(&self.v) {
            Some(ty) => ty.clone().generate_constraints(state).into(),
            None => {
                let fresh = state.fresh_kind_var();
                fresh.into()
            }
        }
    }
}
