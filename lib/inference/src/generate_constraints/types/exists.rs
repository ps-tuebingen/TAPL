use super::{GenState, GenerateConstraints};
use syntax::{kinds::Kind, language::Language, types::Exists};

impl<Lang> GenerateConstraints for Exists<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    type Target = Kind;

    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        todo!()
    }
}
