use crate::{
    syntax::Var,
    to_err,
    types::{Type, TypeVar},
};
use common::errors::{Error, ErrorKind, ErrorLocation};
use std::collections::HashMap;

pub mod lambda;
pub mod lambda_sub;
pub mod nat;
pub mod pack;
pub mod record;
pub mod subtype;
pub mod term;
pub use subtype::check_subtype;

pub fn to_check_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Check)
}

#[derive(Clone, Default)]
pub struct Env {
    pub vars: HashMap<Var, Type>,
    pub ty_vars: HashMap<TypeVar, Type>,
}

impl Env {
    pub fn add_var(&mut self, v: &Var, ty: &Type) {
        self.vars.insert(v.clone(), ty.clone());
    }

    pub fn add_tyvar(&mut self, v: &TypeVar, ty: &Type) {
        self.ty_vars.insert(v.clone(), ty.clone());
    }

    pub fn get_var(&mut self, v: &Var) -> Result<Type, ErrorKind> {
        self.vars
            .get(v)
            .cloned()
            .ok_or(ErrorKind::FreeVariable(v.clone()))
    }

    pub fn get_tyvar(&mut self, v: &TypeVar) -> Result<Type, ErrorKind> {
        self.ty_vars
            .get(v)
            .cloned()
            .ok_or(ErrorKind::FreeTypeVariable(v.clone()))
    }
}
