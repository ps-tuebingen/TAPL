use super::{GenState, GenerateConstraints};
use syntax::{language::Language, terms::Something, types::Optional};

impl<Lang> GenerateConstraints for Something<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Optional<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);
        Optional::new(term_ty, self.span).into()
    }
}
