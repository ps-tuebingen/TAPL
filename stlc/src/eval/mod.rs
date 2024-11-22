use super::{terms::subst::Subst, Var};

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
pub mod tup;
pub mod unit;
pub mod value;
pub mod variant;

pub use value::Value;

pub trait Eval: Subst {
    fn eval(self) -> Option<Value>;
}

impl Eval for Var {
    fn eval(self) -> Option<Value> {
        None
    }
}
