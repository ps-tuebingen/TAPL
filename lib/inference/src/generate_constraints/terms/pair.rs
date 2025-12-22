use super::{GenState, GenerateConstraints};
use syntax::{language::Language, terms::Pair, types::Product};

impl<Lang> GenerateConstraints for Pair<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    Product<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;
    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let fst_ty = self.fst.generate_constraints(state);
        let snd_ty = self.snd.generate_constraints(state);
        Product::new(fst_ty, snd_ty, self.span).into()
    }
}
