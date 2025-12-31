use super::{GenState, GenerateConstraints};
use syntax::{kinds::Kind, language::Language, types::Nat};

impl<Lang> GenerateConstraints for Nat<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    type Target = Kind;

    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        Kind::Star.into()
    }
}
