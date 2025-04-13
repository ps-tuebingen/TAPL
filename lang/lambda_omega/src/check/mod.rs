use crate::{
    kinds::Kind,
    syntax::Var,
    to_err,
    types::{Type, TypeVar},
};
use common::errors::{Error, ErrorKind, ErrorLocation};
use std::collections::HashMap;

pub mod kind;
pub mod types;

pub fn to_check_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Eval)
}

#[derive(Clone, Default)]
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
            .ok_or(to_check_err(ErrorKind::FreeTypeVariable(var.clone())))
            .cloned()
    }

    pub fn add_var(&mut self, var: &Var, ty: &Type) {
        self.vars.insert(var.clone(), ty.clone());
    }

    pub fn get_var(&mut self, var: &Var) -> Result<Type, Error> {
        self.vars
            .get(var)
            .cloned()
            .ok_or(to_check_err(ErrorKind::FreeVariable(var.clone())))
    }
}
