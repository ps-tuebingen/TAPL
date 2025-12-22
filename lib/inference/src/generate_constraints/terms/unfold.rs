use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::Unfold,
    types::{Mu, TypeVariable},
};

impl<Lang> GenerateConstraints for Unfold<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
    Mu<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);
        let mu_var = state.fresh_type_var();
        let body_var = state.fresh_type_var();
        let body_ty = TypeVariable::new(&body_var, self.span);
        let mu_ty = Mu::new(&mu_var, body_ty.clone(), self.span);
        state.add_constraint(EqualityConstraint::new(mu_ty.clone(), term_ty.clone()));
        state.add_constraint(EqualityConstraint::new(body_ty, term_ty));
        mu_ty.into()
    }
}
