use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::Snd,
    types::{Product, TypeVariable},
};

impl<Lang> GenerateConstraints for Snd<Lang>
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
        let fst_var = state.fresh_type_var();
        let fst_ty = TypeVariable::new(&fst_var, self.span);
        let snd_var = state.fresh_type_var();
        let snd_ty = TypeVariable::new(&snd_var, self.span);
        let prod_ty = Product::new(fst_ty, snd_ty.clone(), self.span);
        state.add_constraint(EqualityConstraint::new(prod_ty, term_ty));
        snd_ty.into()
    }
}
