use super::{GenState, GenerateConstraints};
use syntax::{language::Language, terms::Num, types::Nat};

impl<Lang> GenerateConstraints for Num<Lang>
where
    Lang: Language,
    Nat<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        Nat::new(self.span).into()
    }
}
