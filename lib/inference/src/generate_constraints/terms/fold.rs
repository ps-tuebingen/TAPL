use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::Fold,
    types::{Mu, TypeVariable},
};

impl<Lang> GenerateConstraints for Fold<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Mu<Lang>: Into<Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);

        let mu_var = state.fresh_type_var();
        let inner_var = state.fresh_type_var();
        let inner_type_var = TypeVariable::new(&inner_var, self.span);
        let mu_ty = Mu::new(&mu_var, inner_type_var, self.span);

        state.add_constraint(EqualityConstraint::new(mu_ty, self.ty.clone()));
        state.add_constraint(EqualityConstraint::new(term_ty.clone(), self.ty.clone()));
        term_ty
    }
}
