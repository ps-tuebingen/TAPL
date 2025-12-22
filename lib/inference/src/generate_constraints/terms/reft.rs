use super::{GenState, GenerateConstraints};
use syntax::{language::Language, terms::Ref, types::Reference};

impl<Lang> GenerateConstraints for Ref<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Reference<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;
    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);
        let ref_ty = Reference::new(term_ty, self.span);
        ref_ty.into()
    }
}
