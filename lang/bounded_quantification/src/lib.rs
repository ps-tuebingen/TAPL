pub mod examples;
pub mod parser;

pub mod check;
pub mod eval;
pub mod terms;
pub mod types;
pub mod values;

use common::errors::{Error, ErrorKind, ErrorLocation};

pub fn to_err(knd: ErrorKind, loc: ErrorLocation) -> Error {
    Error { kind: knd, loc }
}
