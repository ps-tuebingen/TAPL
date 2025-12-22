use super::{GenState, GenerateConstraints};
use syntax::{language::Language, terms::Cast};

impl<Lang> GenerateConstraints for Cast<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;
    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        self.term.generate_constraints(state);
        self.ty.clone()
    }
}
