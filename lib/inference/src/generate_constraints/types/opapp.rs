use super::{GenState, GenerateConstraints};
use syntax::{kinds::Kind, language::Language, types::OpApp};

impl<Lang> GenerateConstraints for OpApp<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    type Target = Kind;

    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        todo!()
    }
}
