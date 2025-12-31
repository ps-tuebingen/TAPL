use super::{GenState, GenerateConstraints};
use crate::constraints::KindConstraint;
use std::rc::Rc;
use syntax::{kinds::Kind, language::Language, terms::TyApp};

impl<Lang> GenerateConstraints for TyApp<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Lang::Type: GenerateConstraints<Lang = Lang, Target = Kind>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let fun_ty = self.fun.generate_constraints(state);
        let fun_kind = fun_ty.generate_constraints(state);
        let arg_kind = self.arg.generate_constraints(state);
        let from_var = state.fresh_kind_var();
        let from_kind = Kind::Var(from_var);
        let to_var = state.fresh_kind_var();
        let to_kind = Kind::Var(to_var);
        state.add_constraint(KindConstraint::new(from_kind.clone(), arg_kind, self.span));
        state.add_constraint(KindConstraint::new(
            fun_kind,
            Kind::Arrow(Rc::new(from_kind), Rc::new(to_kind)),
            self.span,
        ));
        fun_ty
    }
}
