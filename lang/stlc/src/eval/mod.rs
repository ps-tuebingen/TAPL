use super::{traits::subst::Subst, Var};

pub mod ascription;
pub mod bool;
pub mod errors;
pub mod fix;
pub mod lambda;
pub mod let_exp;
pub mod list;
pub mod nat;
pub mod optional;
pub mod pair;
pub mod record;
pub mod sum;
pub mod term;
pub mod tup;
pub mod unit;
pub mod value;
pub mod variant;

use errors::Error;
pub use value::Value;

pub trait Eval: Subst {
    fn eval(self) -> Result<Value, Error>;
}

impl Eval for Var {
    fn eval(self) -> Result<Value, Error> {
        Err(Error::FreeVariable { var: self })
    }
}

#[cfg(test)]
mod var_tests {
    use super::Eval;
    use crate::syntax::Term;

    #[test]
    fn eval_var() {
        let result = Term::Var("x".to_owned()).eval();
        assert!(result.is_err())
    }
}
