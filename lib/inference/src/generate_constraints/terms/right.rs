use super::{GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::Right,
    types::{Sum, TypeVariable},
};

impl<Lang> GenerateConstraints for Right<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Sum<Lang>: Into<Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let right_ty = self.right_term.generate_constraints(state);
        let left_var = state.fresh_type_var();
        let left_ty = TypeVariable::new(&left_var, self.span);
        Sum::new(left_ty, right_ty, self.span).into()
    }
}
