use crate::Var;
use std::collections::HashSet;

pub mod ascription;
pub mod bool;
pub mod fix;
pub mod lambda;
pub mod let_exp;
pub mod list;
pub mod optional;
pub mod pair;
pub mod record;
pub mod sum;
pub mod term;
pub mod tuple;
pub mod unit;
pub mod variant;

pub trait FreeVars {
    fn free_vars(&self) -> HashSet<Var>;
}

impl FreeVars for Var {
    fn free_vars(&self) -> HashSet<Var> {
        HashSet::from([self.clone()])
    }
}

pub fn fresh_var<T: FreeVars>(t: &T) -> Var {
    let prefix = "x".to_owned();
    let mut num = 0;
    let vars = t.free_vars();
    while vars.contains(&(prefix.clone() + &num.to_string())) {
        num += 1
    }
    prefix + &num.to_string()
}
