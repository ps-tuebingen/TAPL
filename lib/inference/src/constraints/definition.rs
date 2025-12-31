use super::Constraint;
use std::{collections::HashSet, fmt};
use syntax::{TypeVar, language::Language};

#[derive(Clone, Debug)]
pub struct DefConstraints<Lang>
where
    Lang: Language,
{
    pub constraints: Vec<Constraint<Lang>>,
    pub ret_ty: Lang::Type,
    pub used_type_vars: HashSet<TypeVar>,
}

impl<Lang> fmt::Display for DefConstraints<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for constr in &self.constraints {
            writeln!(f, "{constr}")?;
        }
        writeln!(f)?;
        writeln!(f, "Final Type: {}", self.ret_ty)?;
        Ok(())
    }
}
