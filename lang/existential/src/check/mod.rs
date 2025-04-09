use crate::{
    errors::{Error, ErrorKind},
    terms::Var,
    types::{Type, TypeVar},
};
use std::collections::{HashMap, HashSet};

pub mod bool;
pub mod fix;
pub mod lambda;
pub mod nat;
pub mod pack;
pub mod record;
pub mod term;

#[derive(Clone, Default)]
pub struct Env {
    vars: HashMap<Var, Type>,
    ty_vars: HashSet<TypeVar>,
}

impl Env {
    pub fn get_var(&self, v: &Var) -> Result<Type, ErrorKind> {
        self.vars
            .get(v)
            .cloned()
            .ok_or(ErrorKind::FreeVar(v.clone()))
    }

    pub fn add_var(&mut self, v: Var, ty: Type) {
        self.vars.insert(v, ty);
    }

    pub fn add_tyvar(&mut self, ty: TypeVar) {
        self.ty_vars.insert(ty);
    }
}
