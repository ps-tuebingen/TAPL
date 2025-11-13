use crate::{Location, Name, TypeVar, Var, kinds::Kind, language::Language};
use errors::{FreeTypeVariable, FreeVariable, UndefinedLocation};
use std::collections::HashMap;
use std::fmt;

/// Typing environment
#[derive(Clone, Debug)]
pub struct Environment<Lang>
where
    Lang: Language,
{
    /// Types of bound variables
    pub var_bindings: HashMap<Var, Lang::Type>,
    /// Types of definitions in the program
    pub definitions: HashMap<Name, Lang::Type>,
    /// Kinds of bound type variables
    pub tyvar_bindings: HashMap<TypeVar, Kind>,
    /// Supertypes of bound type variables
    pub tyvar_super: HashMap<TypeVar, Lang::Type>,
    /// Types of defined locations
    pub location_bindings: HashMap<Location, Lang::Type>,
}

impl<Lang> Environment<Lang>
where
    Lang: Language,
{
    /// Create a new empty environment
    #[must_use]
    pub fn new() -> Self {
        Self {
            var_bindings: HashMap::new(),
            definitions: HashMap::new(),
            tyvar_bindings: HashMap::new(),
            tyvar_super: HashMap::new(),
            location_bindings: HashMap::new(),
        }
    }

    /// Add a definition to `self`
    pub fn add_definition(&mut self, n: Name, ty: Lang::Type) {
        self.definitions.insert(n, ty);
    }

    /// Add a variable to `self`
    pub fn add_var(&mut self, var: Var, ty: Lang::Type) {
        self.var_bindings.insert(var, ty);
    }

    /// Look up the type of a variable
    /// # Errors
    /// returns an error if the variable was not found
    pub fn get_var(&self, v: &Var) -> Result<Lang::Type, FreeVariable> {
        let mut res = self.var_bindings.get(v);
        if res.is_none() {
            res = self.definitions.get(v);
        }

        res.map_or_else(|| Err(FreeVariable::new(v)), |ty| Ok(ty.clone()))
    }

    /// Add a kinded type variable to `self`
    pub fn add_tyvar_kind(&mut self, v: TypeVar, knd: Kind) {
        self.tyvar_bindings.insert(v, knd);
    }

    /// Look up the kind of a type variable
    /// # Errors
    /// Returns an error if the type variable was not found
    pub fn get_tyvar_kind(&self, v: &TypeVar) -> Result<Kind, FreeTypeVariable> {
        self.tyvar_bindings
            .get(v)
            .cloned()
            .ok_or_else(|| FreeTypeVariable::new(v))
    }

    /// Add a bounded type variable to `self`
    pub fn add_tyvar_super(&mut self, v: TypeVar, sup: Lang::Type) {
        self.tyvar_super.insert(v, sup);
    }

    /// Get the supertype of a type variable
    /// # Errors
    /// returns an error if the type variable was not found
    pub fn get_tyvar_super(&self, v: &TypeVar) -> Result<Lang::Type, FreeTypeVariable> {
        self.tyvar_super
            .get(v)
            .cloned()
            .ok_or_else(|| FreeTypeVariable::new(v))
    }

    /// Add a location to `self`
    pub fn add_loc(&mut self, l: Location, ty: Lang::Type) {
        self.location_bindings.insert(l, ty);
    }

    /// Get the type of a location
    /// # Errors
    /// returns an error if the location was not found
    pub fn get_loc(&self, l: &Location) -> Result<Lang::Type, UndefinedLocation> {
        self.location_bindings
            .get(l)
            .cloned()
            .ok_or_else(|| UndefinedLocation::new(*l))
    }
}

impl<Lang> Default for Environment<Lang>
where
    Lang: Language,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<Lang> fmt::Display for Environment<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (var, ty) in &self.var_bindings {
            write!(f, "{var} : {ty},")?;
        }

        for (nm, ty) in &self.definitions {
            write!(f, "{nm} : {ty},")?;
        }

        for (var, knd) in &self.tyvar_bindings {
            write!(f, "{var} :: {knd}")?;
        }

        for (var, sup) in &self.tyvar_super {
            write!(f, "{var} <: {sup}")?;
        }

        for (loc, ty) in &self.location_bindings {
            write!(f, "{loc} : {ty}")?;
        }
        Ok(())
    }
}
