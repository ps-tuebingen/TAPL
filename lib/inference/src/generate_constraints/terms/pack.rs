use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{language::Language, terms::Pack, types::TypeVariable};

impl<Lang> GenerateConstraints for Pack<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        // TODO: this definitely does not work yet, but because exists can be bounded or not this
        // might be difficult
        let res_var = state.fresh_type_var();
        let res_ty = TypeVariable::new(&res_var, self.span);
        state.add_constraint(EqualityConstraint::new(
            res_ty.clone(),
            self.outer_ty.clone(),
        ));
        res_ty.into()
    }
}
