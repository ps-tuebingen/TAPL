use super::{GenState, GenerateConstraints};
use crate::constraints::KindOrVar;
use syntax::{language::Language, types::Top};

impl<Lang> GenerateConstraints for Top<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    type Target = KindOrVar;

    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        self.kind.clone().into()
    }
}
