use std::{collections::HashSet, mem::replace};
use syntax::{Var, language::Language};

mod constraints;
pub use constraints::Constraint;

mod definition;
mod program;
mod terms;
mod types;

pub struct GenState<Lang>
where
    Lang: Language,
{
    constraints: Vec<Constraint<Lang>>,
    used_vars: HashSet<Var>,
}

impl<Lang> GenState<Lang>
where
    Lang: Language,
{
    pub fn new() -> Self {
        Self {
            constraints: Vec::new(),
            used_vars: HashSet::new(),
        }
    }

    pub fn clear_constraints(&mut self) -> Vec<Constraint<Lang>> {
        replace(&mut self.constraints, Vec::new())
    }

    pub fn add_constraint<C>(&mut self, constraint: C)
    where
        C: Into<Constraint<Lang>>,
    {
        self.constraints.push(constraint.into())
    }
}

pub trait GenerateConstraints {
    type Lang: Language;
    type Target;
    fn generate_constraints(&self, state: &mut GenState<Self::Lang>) -> Self::Target;
}
