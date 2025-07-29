use crate::{Location, Name, TypeVar, Var, kinds::Kind, types::Type};
use errors::{FreeTypeVariable, FreeVariable, UndefinedLocation};
use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Environment<Ty>
where
    Ty: Type,
{
    pub var_bindings: HashMap<Var, Ty>,
    pub definitions: HashMap<Name, Ty>,
    pub tyvar_bindings: HashMap<TypeVar, Kind>,
    pub tyvar_super: HashMap<TypeVar, Ty>,
    pub location_bindings: HashMap<Location, Ty>,
}

impl<Ty> Environment<Ty>
where
    Ty: Type,
{
    pub fn new() -> Environment<Ty> {
        Environment {
            var_bindings: HashMap::new(),
            definitions: HashMap::new(),
            tyvar_bindings: HashMap::new(),
            tyvar_super: HashMap::new(),
            location_bindings: HashMap::new(),
        }
    }

    pub fn add_definition(&mut self, n: Name, ty: Ty) {
        self.definitions.insert(n, ty);
    }

    pub fn add_var(&mut self, var: Var, ty: Ty) {
        self.var_bindings.insert(var, ty);
    }

    pub fn get_var(&self, v: &Var) -> Result<Ty, FreeVariable> {
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

    pub fn add_tyvar_super(&mut self, v: TypeVar, sup: Ty) {
        self.tyvar_super.insert(v, sup);
    }

    pub fn get_tyvar_super(&self, v: &TypeVar) -> Result<Ty, FreeTypeVariable> {
        self.tyvar_super
            .get(v)
            .cloned()
            .ok_or(FreeTypeVariable::new(v))
    }

    pub fn add_loc(&mut self, l: Location, ty: Ty) {
        self.location_bindings.insert(l, ty);
    }

    pub fn get_loc(&self, l: &Location) -> Result<Ty, UndefinedLocation> {
        self.location_bindings
            .get(l)
            .cloned()
            .ok_or(UndefinedLocation::new(*l))
    }
}

impl<Ty> Default for Environment<Ty>
where
    Ty: Type,
{
    fn default() -> Environment<Ty> {
        Environment::new()
    }
}

impl<Ty> fmt::Display for Environment<Ty>
where
    Ty: Type,
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
