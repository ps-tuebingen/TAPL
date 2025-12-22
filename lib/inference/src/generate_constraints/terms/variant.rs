use super::{GenState, GenerateConstraints, VariantConstraint};
use syntax::{language::Language, terms::Variant};

impl<Lang> GenerateConstraints for Variant<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;
    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);
        state.add_constraint(VariantConstraint::new(
            self.ty.clone(),
            &self.label,
            term_ty,
        ));
        self.ty.clone()
    }
}
