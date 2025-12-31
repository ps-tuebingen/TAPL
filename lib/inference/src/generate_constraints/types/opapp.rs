use super::{GenState, GenerateConstraints};
use crate::constraints::KindConstraint;
use std::rc::Rc;
use syntax::{kinds::Kind, language::Language, types::OpApp};

impl<Lang> GenerateConstraints for OpApp<Lang>
where
    Lang: Language,
    Lang::Type: GenerateConstraints<Lang = Lang, Target = Kind>,
{
    type Lang = Lang;
    type Target = Kind;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let fun_kind = self.fun.generate_constraints(state);
        let arg_kind = self.arg.generate_constraints(state);
        let from_var = state.fresh_kind_var();
        let from_kind = Kind::Var(from_var);
        let to_var = state.fresh_kind_var();
        let to_kind = Kind::Var(to_var);
        state.add_constraint(KindConstraint::new(from_kind.clone(), arg_kind, self.span));
        state.add_constraint(KindConstraint::new(
            Kind::Arrow(Rc::new(from_kind), Rc::new(to_kind.clone())),
            fun_kind,
            self.span,
        ));
        to_kind
    }
}
