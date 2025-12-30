use super::{GenState, GenerateConstraints};
use crate::constraints::{KindConstraint, KindOrVar};
use syntax::{kinds::Kind, language::Language, types::Sum};

impl<Lang> GenerateConstraints for Sum<Lang>
where
    Lang: Language,
    Lang::Type: GenerateConstraints<Lang = Lang, Target = KindOrVar>,
{
    type Lang = Lang;
    type Target = KindOrVar;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let left_kind = self.left.generate_constraints(state);
        let right_kind = self.right.generate_constraints(state);
        state.add_constraint(KindConstraint::new(left_kind, Kind::Star, self.span));
        state.add_constraint(KindConstraint::new(right_kind, Kind::Star, self.span));
        Kind::Star.into()
    }
}
