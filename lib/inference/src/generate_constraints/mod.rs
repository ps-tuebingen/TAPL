use std::collections::{HashMap, HashSet};
use syntax::{Location, TypeVar, Var, free_vars::FreeTypeVars, language::Language};

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
    used_type_vars: HashSet<TypeVar>,
    pub var_types: HashMap<Var, Lang::Type>,
    pub loc_types: HashMap<Location, Lang::Type>,
    used_kind_vars: HashSet<String>,
}

impl<Lang> GenState<Lang>
where
    Lang: Language,
{
    pub fn new<U>(used: &U) -> Self
    where
        U: FreeTypeVars,
    {
        let mut vars = HashSet::new();
        used.free_type_vars(&mut vars);
        Self {
            constraints: Vec::new(),
            used_type_vars: vars,
            var_types: HashMap::new(),
            loc_types: HashMap::new(),
            used_kind_vars: HashSet::new(),
        }
    }

    pub fn add_constraint<C>(&mut self, constraint: C)
    where
        C: Into<Constraint<Lang>>,
    {
        self.constraints.push(constraint.into())
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

pub trait GenerateConstraints {
    type Lang: Language;
    type Target;
    fn generate_constraints(&self, state: &mut GenState<Self::Lang>) -> Self::Target;
}
