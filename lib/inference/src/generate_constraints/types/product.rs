use super::{GenState, GenerateConstraints};
use crate::constraints::KindConstraint;
use syntax::{kinds::Kind, language::Language, types::Product};

impl<Lang> GenerateConstraints for Product<Lang>
where
    Lang: Language,
    Lang::Type: GenerateConstraints<Lang = Lang, Target = Kind>,
{
    type Lang = Lang;
    type Target = Kind;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let fst_kind = self.fst.generate_constraints(state);
        let snd_kind = self.snd.generate_constraints(state);
        state.add_constraint(KindConstraint::new(fst_kind, Kind::Star, self.span));
        state.add_constraint(KindConstraint::new(snd_kind, Kind::Star, self.span));
        Kind::Star
    }
}
