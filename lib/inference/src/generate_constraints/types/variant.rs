use super::{GenState, GenerateConstraints};
use crate::constraints::KindConstraint;
use syntax::{kinds::Kind, language::Language, types::Variant};

impl<Lang> GenerateConstraints for Variant<Lang>
where
    Lang: Language,
    Lang::Type: GenerateConstraints<Lang = Lang, Target = Kind>,
{
    type Lang = Lang;
    type Target = Kind;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        for (_, ty) in &self.variants {
            let ty_kind = ty.generate_constraints(state);
            state.add_constraint(KindConstraint::new(ty_kind, Kind::Star, self.span));
        }
        Kind::Star.into()
    }
}
