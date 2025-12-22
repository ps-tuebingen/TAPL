use super::{GenState, GenerateConstraints};
use syntax::{language::Language, terms::TyLambda, types::Forall};

impl<Lang> GenerateConstraints for TyLambda<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Forall<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let term_ty = self.term.generate_constraints(state);
        Forall::new(&self.var, self.annot.clone(), term_ty, self.span).into()
    }
}
