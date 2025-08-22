use crate::{Location, Name, TypeVar, Var, kinds::Kind, language::Language};
use errors::{FreeTypeVariable, FreeVariable, UndefinedLocation};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Environment<Lang>
where
    Lang: Language,
{
    pub var_bindings: HashMap<Var, Lang::Type>,
    pub definitions: HashMap<Name, Lang::Type>,
    pub tyvar_bindings: HashMap<TypeVar, Kind>,
    pub tyvar_super: HashMap<TypeVar, Lang::Type>,
    pub location_bindings: HashMap<Location, Lang::Type>,
}

impl<Lang> Environment<Lang>
where
    Lang: Language,
{
    pub fn new() -> Environment<Lang> {
        Environment {
            var_bindings: HashMap::new(),
            definitions: HashMap::new(),
            tyvar_bindings: HashMap::new(),
            tyvar_super: HashMap::new(),
            location_bindings: HashMap::new(),
        }
    }

    pub fn add_definition(&mut self, n: Name, ty: Lang::Type) {
        self.definitions.insert(n, ty);
    }

    pub fn add_var(&mut self, var: Var, ty: Lang::Type) {
        self.var_bindings.insert(var, ty);
    }

    pub fn get_var(&self, v: &Var) -> Result<Lang::Type, FreeVariable> {
        let mut res = self.var_bindings.get(v);
        if res.is_none() {
            res = self.definitions.get(v);
        }

        match res {
            Some(ty) => Ok(ty.clone()),
            None => Err(FreeVariable::new(v)),
        }
    }

    pub fn add_tyvar_kind(&mut self, v: TypeVar, knd: Kind) {
        self.tyvar_bindings.insert(v, knd);
    }

    pub fn get_tyvar_kind(&self, v: &TypeVar) -> Result<Kind, FreeTypeVariable> {
        self.tyvar_bindings
            .get(v)
            .cloned()
            .ok_or(FreeTypeVariable::new(v))
    }

    pub fn add_tyvar_super(&mut self, v: TypeVar, sup: Lang::Type) {
        self.tyvar_super.insert(v, sup);
    }

    pub fn get_tyvar_super(&self, v: &TypeVar) -> Result<Lang::Type, FreeTypeVariable> {
        self.tyvar_super
            .get(v)
            .cloned()
            .ok_or(FreeTypeVariable::new(v))
    }

    pub fn add_loc(&mut self, l: Location, ty: Lang::Type) {
        self.location_bindings.insert(l, ty);
    }

    pub fn get_loc(&self, l: &Location) -> Result<Lang::Type, UndefinedLocation> {
        self.location_bindings
            .get(l)
            .cloned()
            .ok_or(UndefinedLocation::new(*l))
    }
}

impl<Lang> Default for Environment<Lang>
where
    Lang: Language,
{
    fn default() -> Environment<Lang> {
        Environment::new()
    }
}

impl<Lang> fmt::Display for Environment<Lang>
where
    Lang: Language,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (var, ty) in self.var_bindings.iter() {
            write!(f, "{var} : {ty},")?;
        }

        for (nm, ty) in self.definitions.iter() {
            write!(f, "{nm} : {ty},")?;
        }

        for (var, knd) in self.tyvar_bindings.iter() {
            write!(f, "{var} :: {knd}")?;
        }

        for (var, sup) in self.tyvar_super.iter() {
            write!(f, "{var} <: {sup}")?;
        }

        for (loc, ty) in self.location_bindings.iter() {
            write!(f, "{loc} : {ty}")?;
        }
        Ok(())
    }
}
