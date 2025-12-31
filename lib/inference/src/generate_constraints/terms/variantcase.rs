use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::{VariantCase, variantcase::VariantPattern},
    types::{TypeVariable, Variant as VariantTy},
};

impl<Lang> GenerateConstraints for VariantCase<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
    VariantTy<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let bound_ty = self.bound_term.generate_constraints(state);
        let mut variants = Vec::with_capacity(self.patterns.len());
        let mut result_ty = None;
        for pt in &self.patterns {
            let rhs_ty = pt.generate_constraints(state);
            match result_ty {
                None => result_ty = Some(rhs_ty.clone()),
                Some(ref ty) => {
                    state.add_constraint(EqualityConstraint::new(rhs_ty.clone(), ty.clone()));
                }
            }

            variants.push((pt.label.clone(), rhs_ty));
        }

        let var_ty = VariantTy::new(variants.into_iter().collect(), self.span);
        state.add_constraint(EqualityConstraint::new(var_ty, bound_ty));
        result_ty.unwrap()
    }
}

impl<Lang> GenerateConstraints for VariantPattern<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TypeVariable<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let var_ty = state.fresh_type_var();
        let ty_var = TypeVariable::new(&var_ty, self.span);
        state.add_var(&self.bound_var, ty_var);

        let res_ty = self.rhs.generate_constraints(state);
        state.var_types.remove(&self.bound_var);
        res_ty
    }
}
