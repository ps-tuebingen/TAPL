use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{language::Language, terms::Try};

impl<Lang> GenerateConstraints for Try<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);
        let handler_ty = self.handler.generate_constraints(state);
        state.add_constraint(EqualityConstraint::new(term_ty.clone(), handler_ty));
        term_ty
    }
}
