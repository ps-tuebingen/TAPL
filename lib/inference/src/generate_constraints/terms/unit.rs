use super::{GenState, GenerateConstraints};
use syntax::{language::Language, terms::Unit, types::Unit as UnitTy};

impl<Lang> GenerateConstraints for Unit<Lang>
where
    Lang: Language,
    UnitTy<Lang>: Into<Lang::Type>,
{
    type Lang = Lang;
    type Target = Lang::Type;

    fn generate_constraints(&self, _: &mut GenState<Lang>) -> Self::Target {
        UnitTy::new(self.span).into()
    }
}
