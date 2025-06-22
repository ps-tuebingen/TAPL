use super::{Location, TypeVar, Var, kinds::Kind, types::Type};
use common::errors::{FreeTypeVariable, FreeVariable, UndefinedLocation};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Environment<Ty>
where
    Ty: Type,
{
    pub var_bindings: HashMap<Var, Ty>,
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
            tyvar_bindings: HashMap::new(),
            tyvar_super: HashMap::new(),
            location_bindings: HashMap::new(),
        }
    }

    pub fn add_var(&mut self, var: Var, ty: Ty) {
        self.var_bindings.insert(var, ty);
    }

    pub fn get_var(&self, v: &Var) -> Result<Ty, FreeVariable> {
        self.var_bindings
            .get(v)
            .cloned()
            .ok_or(FreeVariable::new(v))
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
