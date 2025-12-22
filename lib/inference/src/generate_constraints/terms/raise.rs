use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{language::Language, terms::Raise};

impl<Lang> GenerateConstraints for Raise<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let ex_ty = self.exception.generate_constraints(state);
        state.add_constraint(EqualityConstraint::new(ex_ty, self.exception_ty.clone()));
        self.cont_ty.clone()
    }
}
