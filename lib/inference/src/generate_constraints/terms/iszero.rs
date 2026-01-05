use super::{EqualityConstraint, GenState, GenerateConstraints};
use syntax::{
    language::Language,
    terms::IsZero,
    types::{Bool, Nat},
};

impl<Lang> GenerateConstraints for IsZero<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Nat<Lang>: Into<Lang::Type>,
    Bool<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);
        state.add_constraint(EqualityConstraint::new(term_ty, Nat::new(self.span)));
        Bool::new(self.span).into()
    }
}
