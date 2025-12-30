use super::{GenState, GenerateConstraints};
use crate::constraints::KindOrVar;
use syntax::{kinds::Kind, language::Language, types::Bool};

impl<Lang> GenerateConstraints for Bool<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    type Target = KindOrVar;

    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        Kind::Star.into()
    }
}
