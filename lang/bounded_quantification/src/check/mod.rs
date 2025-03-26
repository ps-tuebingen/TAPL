use crate::{
    errors::{Error, ErrorKind},
    syntax::Var,
    types::{Type, TypeVar},
};
use std::collections::HashMap;

pub mod lambda;
pub mod lambda_sub;
pub mod nat;
pub mod pack;
pub mod subtype;
pub mod term;
pub use subtype::check_subtype;

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
            .ok_or(ErrorKind::FreeVar(v.clone()))
    }

    pub fn get_tyvar(&mut self, v: &TypeVar) -> Result<Type, ErrorKind> {
        self.ty_vars
            .get(v)
            .cloned()
            .ok_or(ErrorKind::FreeTypeVar(v.clone()))
    }
}

pub trait Check {
    fn check(&self, env: &mut Env) -> Result<Type, Error>;
}
