use super::{GenState, GenerateConstraints};
use syntax::{language::Language, terms::Tuple, types::Tuple as TupleTy};

impl<Lang> GenerateConstraints for Tuple<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang, Target = Lang::Type>,
    TupleTy<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, state: &mut GenState<Lang>) -> Self::Target {
        let mut tys = Vec::with_capacity(self.terms.len());
        for t in self.terms.iter() {
            let ty = t.generate_constraints(state);
            tys.push(ty);
        }
        TupleTy::new(tys, self.span).into()
    }
}
