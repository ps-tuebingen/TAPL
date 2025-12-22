use super::{GenState, GenerateConstraints};
use syntax::{language::Language, terms::False, types::Bool};

impl<Lang> GenerateConstraints for False<Lang>
where
    Lang: Language,
    Bool<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        Bool::new(self.span).into()
    }
}
