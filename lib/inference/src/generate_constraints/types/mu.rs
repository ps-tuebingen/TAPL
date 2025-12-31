use super::{GenState, GenerateConstraints};
use syntax::{kinds::Kind, language::Language, types::Mu};

impl<Lang> GenerateConstraints for Mu<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    type Target = Kind;

    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        todo!()
    }
}
