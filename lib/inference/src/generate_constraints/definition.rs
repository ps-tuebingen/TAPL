use super::{ GenerateConstraints,GenState};
use syntax::{definition::Definition, language::Language};

impl<Lang> GenerateConstraints for Definition<Lang>
where
    Lang: Language,
    Lang::Term: GenerateConstraints<Lang = Lang>,
    Lang::Type: GenerateConstraints<Lang = Lang>,
{
    type Lang = Lang;
    type Target = ()
    fn generate_constraints(&self,state:&mut GenState<Self::Lang>) -> Self::Target {
        self.annot.generate_constraints(state);
        self.body.generate_constraints(state);
        ()
    }
}
