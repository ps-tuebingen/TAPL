use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::Deref,
    types::{Reference, TypeVariable},
};

impl<Lang> GenerateConstraints for Deref<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
    Reference<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);
        let arg_var = state.fresh_type_var();
        let var_ty = TypeVariable::new(&arg_var, self.span);
        let ref_ty = Reference::new(var_ty.clone(), self.span);
        state.add_constraint(EqualityConstraint::new(term_ty, ref_ty.into()));
        var_ty.into()
    }
}
