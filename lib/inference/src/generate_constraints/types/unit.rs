use super::{GenState, GenerateConstraints};
use crate::constraints::KindOrVar;
use syntax::{kinds::Kind, language::Language, types::Unit};

impl<Lang> GenerateConstraints for Unit<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    type Target = KindOrVar;

    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        Kind::Star.into()
    }
}
