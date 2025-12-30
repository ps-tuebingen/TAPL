use super::{GenState, GenerateConstraints};
use syntax::{language::Language, untyped::Untyped};

impl<Lang> GenerateConstraints for Untyped<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    type Target = ();
    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {}
}
