use super::{GenState, GenerateConstraints, SubtypeConstraint};
use syntax::{
    language::Language,
    terms::LambdaSub,
    types::{ForallBounded, TypeVariable},
};

impl<Lang> GenerateConstraints for LambdaSub<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
    ForallBounded<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        state.add_constraint(SubtypeConstraint::new(
            TypeVariable::new(&self.var, self.span),
            self.sup_ty.clone(),
        ));
        let body_ty = self.body.generate_constraints(state);
        ForallBounded::new(&self.var, self.sup_ty.clone(), body_ty, self.span).into()
    }
}
