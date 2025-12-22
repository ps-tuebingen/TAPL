use super::{GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::UntypedLambda,
    types::{Fun, TypeVariable},
};

impl<Lang> GenerateConstraints for UntypedLambda<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
    Fun<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let var_var = state.fresh_type_var();
        let var_ty = TypeVariable::new(&var_var, self.span);
        state.add_var(&self.var, var_ty.clone());

        let body_ty = self.body.generate_constraints(state);
        Fun::new(var_ty, body_ty, self.span).into()
    }
}
