use super::{Constraint, GenerateConstraints};
use std::collections::HashSet;
use syntax::{definition::Definition, language::Language};

impl<Lang> GenerateConstraints for Definition<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang>,
    Lang::Type: GenerateConstraints<Lang = Lang>,
{
    type Lang = Lang;
    fn generate_constraints(&self) -> Vec<Constraint<Self::Lang>> {
        let mut constraints = self.annot.generate_constraints();
        constraints.extend(self.body.generate_constraints());
        constraints
    }
}
