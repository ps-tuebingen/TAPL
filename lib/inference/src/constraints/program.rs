use super::DefConstraints;
use std::{collections::HashMap, fmt};
use syntax::{Name, language::Language};

#[derive(Clone, Debug)]
pub struct ProgramConstraints<Lang>
where
    Lang: Language,
{
    pub def_constraints: HashMap<Name, DefConstraints<Lang>>,
    pub main_constraints: DefConstraints<Lang>,
}

impl<Lang> fmt::Display for ProgramConstraints<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "=== Generated Constraints ===")?;
        for (name, def_constr) in &self.def_constraints {
            writeln!(f, "{name}")?;
            write!(f, "{def_constr}")?;
        }

        writeln!(f, "main")?;
        write!(f, "{}", self.main_constraints)
    }
}
