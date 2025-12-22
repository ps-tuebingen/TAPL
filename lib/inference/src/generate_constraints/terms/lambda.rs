use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::Lambda,
    types::{Fun, TypeVariable},
};

impl<Lang> GenerateConstraints for Lambda<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
    Fun<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let bound_var = state.fresh_type_var();
        let bound_ty = TypeVariable::new(&bound_var, self.span);
        state.add_var(&self.var, bound_ty.clone());

        let body_ty = self.body.generate_constraints(state);
        state.add_constraint(EqualityConstraint::new(
            self.annot.clone(),
            bound_ty.clone(),
        ));
        Fun::new(bound_ty, body_ty, self.span).into()
    }
}
