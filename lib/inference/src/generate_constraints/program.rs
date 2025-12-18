use super::{Constraint, GenerateConstraints};
use syntax::{language::Language, program::Program};

impl<Lang> GenerateConstraints for Program<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang>,
    Lang::Type: GenerateConstraints<Lang = Lang>,
{
    type Lang = Lang;
    fn generate_constraints(&self) -> Vec<Constraint<Self::Lang>> {
        let mut constraints = self.main.generate_constraints();
        for def in self.definitions.iter() {
            constraints.extend(def.generate_constraints());
        }
        constraints
    }
}
