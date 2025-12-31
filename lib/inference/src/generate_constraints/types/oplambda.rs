use super::{GenState, GenerateConstraints};
use syntax::{kinds::Kind, language::Language, types::OpLambda};

impl<Lang> GenerateConstraints for OpLambda<Lang>
where
    Lang: Language,
    Lang::Type: GenerateConstraints<Lang = Lang, Target = Kind>,
{
    type Lang = Lang;
    type Target = Kind;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let inner_kind = self.body.generate_constraints(state);
        inner_kind.abs()
    }
}
