use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::TryWithVal,
    types::{Fun, TypeVariable},
};

impl<Lang> GenerateConstraints for TryWithVal<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Fun<Lang>: Into<Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);
        let handler_ty = self.handler.generate_constraints(state);

        let from_var = state.fresh_type_var();
        let to_var = state.fresh_type_var();
        let from_ty = TypeVariable::new(&from_var, self.span);
        let to_ty = TypeVariable::new(&to_var, self.span);
        let fun_ty = Fun::new(from_ty, to_ty.clone(), self.span);

        state.add_constraint(EqualityConstraint::new(handler_ty, fun_ty));
        state.add_constraint(EqualityConstraint::new(to_ty, term_ty.clone()));
        term_ty
    }
}
