use super::{terms::Var, to_err, types::Type};
use common::errors::{Error, ErrorKind, ErrorLocation};
use std::collections::HashMap;

pub mod bool;
pub mod fix;
pub mod fold;
pub mod lambda;
pub mod let_exp;
pub mod nat;
pub mod pair;
pub mod record;
pub mod terms;
pub mod variant;

pub fn to_check_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Check)
}

#[derive(Clone, Default)]
pub struct Env(HashMap<Var, Type>);

impl Env {
    pub fn get(&self, v: &Var) -> Result<Type, ErrorKind> {
        self.0
            .get(v)
            .ok_or(ErrorKind::FreeVariable(v.clone()))
            .cloned()
    }

    pub fn insert(&mut self, v: Var, ty: Type) {
        self.0.insert(v, ty);
    }
}
