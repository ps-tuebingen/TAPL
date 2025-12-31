use crate::constraints::Constraint;
use std::collections::{HashMap, HashSet, VecDeque};
use syntax::{TypeVar, kinds::Kind, language::Language};

/// State during constrint solving
/// used in [`crate::SolveConstraint`]
pub struct SolveState<Lang>
where
    Lang: Language,
{
    /// Currently unsolved constraints
    remaining_constraints: VecDeque<Constraint<Lang>>,
    /// Names of used type variables
    used_type_vars: HashSet<TypeVar>,
    /// Types of type variables
    pub var_tys: HashMap<TypeVar, Lang::Type>,
    /// Supertypes of type variables
    pub tyvar_super: HashMap<TypeVar, Lang::Type>,
    /// Kinds of kind variables
    pub kind_vars: HashMap<String, Kind>,
}

impl<Lang> SolveState<Lang>
where
    Lang: Language,
{
    /// Create a new state from given constraints and used variables
    #[must_use]
    pub fn new(constraints: Vec<Constraint<Lang>>, used: HashSet<TypeVar>) -> Self {
        Self {
            remaining_constraints: constraints.into_iter().collect(),
            var_tys: HashMap::new(),
            tyvar_super: HashMap::new(),
            kind_vars: HashMap::new(),
            used_type_vars: used,
        }
    }

    pub fn add_tyvar<Ty>(&mut self, var: &str, super_ty: Ty)
    where
        Ty: Into<Lang::Type>,
    {
        self.tyvar_super.insert(var.to_string(), super_ty.into());
    }

    pub fn add_constraint<C>(&mut self, c: C)
    where
        C: Into<Constraint<Lang>>,
    {
        self.remaining_constraints.push_back(c.into());
    }

    pub fn next_constraint(&mut self) -> Option<Constraint<Lang>> {
        self.remaining_constraints.pop_front()
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
}
