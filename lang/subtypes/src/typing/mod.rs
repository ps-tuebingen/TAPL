use crate::{
    syntax::{Loc, Var},
    types::Type,
};
use std::collections::HashMap;

pub mod errors;
pub mod join;
pub mod meet;
pub mod subtyping;

pub mod bool;
pub mod cast;
pub mod fix;
pub mod lambda;
pub mod let_exp;
pub mod list;
pub mod nat;
pub mod record;
pub mod references;
pub mod terms;
pub mod unit;
pub mod variant;

use errors::Error;
pub use join::join;
pub use meet::meet;
pub use subtyping::is_subtype;

#[derive(Clone, Default)]
pub struct TypingContext {
    var_env: HashMap<Var, Type>,
    store_typing: HashMap<Loc, Type>,
}

impl TypingContext {
    pub fn get_var(&self, v: &str) -> Result<Type, Error> {
        self.var_env
            .get(v)
            .cloned()
            .ok_or(Error::FreeVar(v.to_owned()))
    }

    pub fn add_var(&mut self, var: &str, ty: &Type) {
        self.var_env.insert(var.to_owned(), ty.clone());
    }

    pub fn lookup_location(&self, loc: Loc) -> Option<Type> {
        self.store_typing.get(&loc).cloned()
    }
}
