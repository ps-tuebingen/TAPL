use super::{GenState, GenerateConstraints};
use std::rc::Rc;
use syntax::{kinds::Kind, language::Language, types::OpLambdaSub};

impl<Lang> GenerateConstraints for OpLambdaSub<Lang>
where
    Lang: Language,
    Lang::Type: GenerateConstraints<Lang = Lang, Target = Kind>,
{
    type Lang = Lang;
    type Target = Kind;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        state.add_var(&self.var, Rc::unwrap_or_clone(self.sup.clone()));
        let body_kind = self.body.generate_constraints(state);
        body_kind.abs()
    }
}
