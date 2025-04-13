use super::to_err;
use common::errors::{Error, ErrorKind, ErrorLocation};

pub mod ascription;
pub mod bool;
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

pub use value::Value;

pub fn to_eval_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Eval)
}

#[cfg(test)]
mod var_tests {
    use crate::syntax::Term;
    use common::Eval;

    #[test]
    fn eval_var() {
        let result = Term::Var("x".to_owned()).eval(Default::default());
        assert!(result.is_err())
    }
}
