use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::Fst,
    types::{Product, TypeVariable},
};

impl<Lang> GenerateConstraints for Fst<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
    Product<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);
        let left_var = state.fresh_type_var();
        let right_var = state.fresh_type_var();
        let left_ty = TypeVariable::new(&left_var, self.span);
        let right_ty = TypeVariable::new(&right_var, self.span);
        let product_ty = Product::new(left_ty.clone(), right_ty, self.span);
        state.add_constraint(EqualityConstraint::new(term_ty, product_ty));
        left_ty.into()
    }
}
