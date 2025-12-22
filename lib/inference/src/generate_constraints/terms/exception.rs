use super::{GenState, GenerateConstraints};
use syntax::{language::Language, terms::Exception};

impl<Lang> GenerateConstraints for Exception<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    type Target = Lang::Type;
    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        self.ty.clone()
    }
}
