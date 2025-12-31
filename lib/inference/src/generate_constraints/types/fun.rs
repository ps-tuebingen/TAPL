use super::{GenState, GenerateConstraints};
use crate::constraints::KindConstraint;
use syntax::{kinds::Kind, language::Language, types::Fun};

impl<Lang> GenerateConstraints for Fun<Lang>
where
    Lang: Language,
    Lang::Type: GenerateConstraints<Lang = Lang, Target = Kind>,
{
    type Lang = Lang;
    type Target = Kind;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let from_kind = self.from.generate_constraints(state);
        let to_kind = self.to.generate_constraints(state);
        state.add_constraint(KindConstraint::new(from_kind, Kind::Star, self.span));
        state.add_constraint(KindConstraint::new(to_kind, Kind::Star, self.span));
        Kind::Star.into()
    }
}
