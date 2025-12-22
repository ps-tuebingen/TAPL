use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{language::Language, terms::If, types::Bool};

impl<Lang> GenerateConstraints for If<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Bool<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let cond_ty = self.if_cond.generate_constraints(state);
        let then_ty = self.then_term.generate_constraints(state);
        let else_ty = self.else_term.generate_constraints(state);

        state.add_constraint(EqualityConstraint::new(cond_ty, Bool::new(self.span)));
        state.add_constraint(EqualityConstraint::new(then_ty, else_ty.clone()));
        else_ty
    }
}
