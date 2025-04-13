pub mod lambda;
pub mod terms;
pub mod tylambda;
pub mod value;

pub use value::Value;

use super::to_err;
use common::errors::{Error, ErrorKind, ErrorLocation};

pub fn to_eval_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Eval)
}
