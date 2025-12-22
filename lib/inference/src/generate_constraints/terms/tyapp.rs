use super::{GenState, GenerateConstraints};
use syntax::{language::Language, terms::TyApp};

impl<Lang> GenerateConstraints for TyApp<Lang>
where
    Lang: Language,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        todo!()
    }
}
