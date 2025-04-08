use crate::{
    errors::ErrorKind,
    syntax::{
        kinds::Kind,
        terms::Var,
        types::{Type, TypeVar},
    },
};
use std::collections::HashMap;

pub mod terms;
pub mod types;

#[derive(Clone, Default)]
pub struct Env {
    pub vars: HashMap<Var, Type>,
    pub ty_vars: HashMap<TypeVar, Kind>,
}

impl Env {
    pub fn get_var(&self, v: &Var) -> Result<Type, ErrorKind> {
        self.vars
            .get(v)
            .cloned()
            .ok_or(ErrorKind::FreeVar(v.clone()))
    }

    pub fn add_var(&mut self, v: &Var, ty: &Type) {
        self.vars.insert(v.clone(), ty.clone());
    }

    pub fn get_tyvar(&self, v: &TypeVar) -> Result<Kind, ErrorKind> {
        self.ty_vars
            .get(v)
            .cloned()
            .ok_or(ErrorKind::FreeTypeVar(v.clone()))
    }

    pub fn add_tyvar(&mut self, v: &TypeVar, kind: &Kind) {
        self.ty_vars.insert(v.clone(), kind.clone());
    }
}
