use super::{Constraint, EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::App,
    types::{Fun, TypeVariable},
};

impl<Lang> GenerateConstraints for App<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;
    fn generate_constraints(&self, state: &mut GenState<Self::Lang>) -> Self::Target {
        let fun_ty = self.fun.generate_constraints(state);
        let arg_ty = self.arg.generate_constraints(state);
        let to_var = TypeVariable::new();
        state.add_constraint(EqualityConstraint::new(fun_ty, Fun::new()));
        todo!()
    }
}
