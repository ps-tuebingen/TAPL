use super::{GenState, GenerateConstraints};
use crate::constraints::KindConstraint;
use std::rc::Rc;
use syntax::{kinds::Kind, language::Language, types::ForallBounded};

impl<Lang> GenerateConstraints for ForallBounded<Lang>
where
    Lang: Language,
    Lang::Type: GenerateConstraints<Lang = Lang, Target = Kind>,
{
    type Lang = Lang;
    type Target = Kind;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        state.add_var(&self.var, Rc::unwrap_or_clone(self.sup_ty.clone()));
        let inner_kind = self.ty.generate_constraints(state);
        state.add_constraint(KindConstraint::new(inner_kind, Kind::Star, self.span));
        Kind::Star
    }
}
