use super::{GenState, GenerateConstraints};
use syntax::{kinds::Kind, language::Language, types::Bot};

impl<Lang> GenerateConstraints for Bot<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    type Target = Kind;

    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        self.kind.clone().into()
    }
}
