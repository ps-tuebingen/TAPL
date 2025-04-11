pub mod app;
pub mod bool;
pub mod fix;
pub mod lambda;
pub mod nat;
pub mod pack;
pub mod record;
pub mod record_proj;
pub mod terms;
pub mod tyapp;
pub mod tylambda;
pub mod unpack;
pub mod value;
pub use value::Value;

use super::to_err;
use common::errors::{Error, ErrorKind, ErrorLocation};

pub fn to_eval_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Eval)
}
