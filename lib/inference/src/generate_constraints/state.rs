use crate::constraints::Constraint;
use std::collections::{HashMap, HashSet};
use syntax::{Location, TypeVar, Var, language::Language};

/// State of Generating Constraints
/// used in [`crate::GenerateConstraints`]
pub struct GenState<Lang>
where
    Lang: Language,
{
    /// Generated Constraints
    pub constraints: Vec<Constraint<Lang>>,
    /// Names of type variables that are already in use
    pub used_type_vars: HashSet<TypeVar>,
    /// Names of kind variables that are already in use
    used_kind_vars: HashSet<String>,
    /// Types of variables
    pub var_types: HashMap<Var, Lang::Type>,
    /// Types of memory Locations
    pub loc_types: HashMap<Location, Lang::Type>,
}

impl<Lang> GenState<Lang>
where
    Lang: Language,
{
    /// Crate a new state with given used variables
    #[must_use]
    pub fn new(used: HashSet<TypeVar>) -> Self
where {
        Self {
            constraints: Vec::new(),
            used_type_vars: used,
            var_types: HashMap::new(),
            loc_types: HashMap::new(),
            used_kind_vars: HashSet::new(),
        }
    }

    pub fn add_constraint<C>(&mut self, constraint: C)
    where
        C: Into<Constraint<Lang>>,
    {
        self.constraints.push(constraint.into());
    }

    pub fn fresh_type_var(&mut self) -> TypeVar {
        let mut num = 0;
        let mut var = format!("X{num}");
        while self.used_type_vars.contains(&var) {
            num += 1;
            var = format!("X{num}");
        }
        self.used_type_vars.insert(var.clone());
        var
    }

    pub fn fresh_kind_var(&mut self) -> String {
        let mut num = 0;
        let mut var = format!("K{num}");
        while self.used_kind_vars.contains(&var) {
            num += 1;
            var = format!("K{num}");
        }
        self.used_kind_vars.insert(var.clone());
        var
    }

    pub fn add_var<Ty>(&mut self, var: &str, ty: Ty)
    where
        Ty: Into<Lang::Type>,
    {
        self.var_types.insert(var.to_string(), ty.into());
    }

    pub fn add_loc<Ty>(&mut self, loc: Location, ty: Ty)
    where
        Ty: Into<Lang::Type>,
    {
        self.loc_types.insert(loc, ty.into());
    }
}
