pub mod terms;
pub mod types;
pub mod value;
use super::to_err;
use crate::Env;
use common::errors::{Error, ErrorKind, ErrorLocation};
pub use value::Value;

pub fn to_eval_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Eval)
}
