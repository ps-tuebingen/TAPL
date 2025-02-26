use crate::{
    errors::{Error, ErrorKind},
    terms::Var,
    types::{Type, TypeVar},
};
use std::collections::{HashMap, HashSet};

pub mod bool;
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
    pub fn add_var(&mut self, v: Var, ty: Type) {
        self.vars.insert(v, ty);
    }

    pub fn add_tyvar(&mut self, ty: TypeVar) {
        self.ty_vars.insert(ty);
    }
}

pub trait Check {
    fn check(&self, env: &mut Env) -> Result<Type, Error>;
}

impl Check for Var {
    fn check(&self, env: &mut Env) -> Result<Type, Error> {
        env.vars
            .get(self)
            .ok_or(Error::check(ErrorKind::var(self), &self.as_str()))
            .cloned()
    }
}
