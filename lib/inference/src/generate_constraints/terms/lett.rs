use super::{GenState, GenerateConstraints};
use syntax::{language::Language, terms::Let};

impl<Lang> GenerateConstraints for Let<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;
    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let bound_ty = self.bound_term.generate_constraints(state);
        state.add_var(&self.var, bound_ty);
        self.in_term.generate_constraints(state)
    }
}
