use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{language::Language, terms::Ascribe};

impl<Lang> GenerateConstraints for Ascribe<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Self::Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);
        state.add_constraint(EqualityConstraint::new(term_ty, self.ty.clone()));
        self.ty.clone()
    }
}
