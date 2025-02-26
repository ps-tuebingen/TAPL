use super::{
    errors::{Error, ErrorKind},
    terms::Var,
    types::Type,
};
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

#[derive(Clone, Default)]
pub struct Env(HashMap<Var, Type>);

impl Env {
    pub fn get(&self, v: &Var) -> Result<Type, ErrorKind> {
        self.0.get(v).ok_or(ErrorKind::FreeVar(v.clone())).cloned()
    }

    pub fn insert(&mut self, v: Var, ty: Type) {
        self.0.insert(v, ty);
    }
}

pub trait Check {
    fn check(&self, env: &mut Env) -> Result<Type, Error>;
}
