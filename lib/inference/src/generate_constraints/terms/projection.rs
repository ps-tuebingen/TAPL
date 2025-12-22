use super::{GenState, GenerateConstraints, IndexConstraint};
use syntax::{language::Language, terms::Projection, types::TypeVariable};

impl<Lang> GenerateConstraints for Projection<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;
    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);
        let res_var = state.fresh_type_var();
        let res_ty = TypeVariable::new(&res_var, self.span);
        state.add_constraint(IndexConstraint::new(term_ty, self.index, res_ty.clone()));
        res_ty.into()
    }
}
