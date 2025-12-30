use super::{GenState, GenerateConstraints};
use syntax::{kinds::Kind, language::Language, types::Unit};

impl<Lang> GenerateConstraints for Unit<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    type Target = Kind;

    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        Kind::Star
    }
}
