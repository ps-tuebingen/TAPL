use super::{GenState, GenerateConstraints};
use crate::constraints::KindOrVar;
use syntax::{language::Language, types::OpLambdaSub};

impl<Lang> GenerateConstraints for OpLambdaSub<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    type Target = KindOrVar;

    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        todo!()
    }
}
