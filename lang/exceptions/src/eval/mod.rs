pub mod app;
pub mod errt;
pub mod ift;
pub mod iszero;
pub mod lambda;
pub mod pred;
pub mod raise;
pub mod succ;
pub mod terms;
pub mod tryt;
pub mod tryval;
pub mod unit;
pub mod values;
use super::to_err;
use common::errors::{Error, ErrorKind, ErrorLocation};
use values::Value;

pub fn to_eval_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Eval)
}

#[cfg(test)]
mod eval_tests {
    use super::Value;
    use crate::syntax::term_tests::{example_term1, example_term2};
    use common::Eval;

    #[test]
    fn eval1() {
        let result = example_term1().eval(Default::default()).unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }

    #[test]
    fn eval2() {
        let result = example_term2().eval(Default::default()).unwrap();
        let expected = Value::Unit;
        assert_eq!(result, expected)
    }
}
