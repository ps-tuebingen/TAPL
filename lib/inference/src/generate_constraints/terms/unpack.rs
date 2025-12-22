use super::{GenState, GenerateConstraints};
use syntax::{language::Language, terms::Unpack};

impl<Lang> GenerateConstraints for Unpack<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        todo!()
    }
}
