pub mod terms;
pub mod types;
use super::{to_err, Env};
use common::errors::{Error, ErrorKind, ErrorLocation};
pub use types::subtype::check_subtype;

pub fn to_check_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Check)
}

pub fn to_kind_err(knd: ErrorKind) -> Error {
    to_err(knd, ErrorLocation::Kind)
}
