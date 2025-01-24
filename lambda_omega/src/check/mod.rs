use crate::{
    errors::Error,
    kinds::Kind,
    syntax::Var,
    types::{Type, TypeVar},
};
use std::collections::HashMap;

pub mod kind;
pub mod types;
pub use kind::kind_ty;

#[derive(Clone)]
pub struct Env {
    pub vars: HashMap<Var, Type>,
    pub ty_vars: HashMap<TypeVar, Kind>,
}

impl Env {
    pub fn add_tyvar(&mut self, var: &TypeVar, kind: &Kind) {
        self.ty_vars.insert(var.clone(), kind.clone());
    }

    pub fn get_tyvar(&mut self, var: &TypeVar) -> Result<Kind, Error> {
        self.ty_vars
            .get(var)
            .ok_or(Error::FreeTypeVar(var.clone()))
            .cloned()
    }

    pub fn add_var(&mut self, var: &Var, ty: &Type) {
        self.vars.insert(var.clone(), ty.clone());
    }

    pub fn get_var(&mut self, var: &Var) -> Result<Type, Error> {
        self.vars
            .get(var)
            .cloned()
            .ok_or(Error::FreeVar(var.clone()))
    }
}
