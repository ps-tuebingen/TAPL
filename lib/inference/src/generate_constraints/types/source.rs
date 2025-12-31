use super::{GenState, GenerateConstraints};
use crate::constraints::KindConstraint;
use syntax::{kinds::Kind, language::Language, types::Source};

impl<Lang> GenerateConstraints for Source<Lang>
where
    Lang: Language,
    Lang::Type: GenerateConstraints<Lang = Lang, Target = Kind>,
{
    type Lang = Lang;
    type Target = Kind;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let inner_kind = self.ty.generate_constraints(state);
        state.add_constraint(KindConstraint::new(inner_kind, Kind::Star, self.span));
        Kind::Star.into()
    }
}
