pub mod check;
pub mod eval;
pub mod parser;
pub mod terms;
pub mod types;

use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    langs::Lang,
};

pub fn to_err(knd: ErrorKind, loc: ErrorLocation) -> Error {
    Error {
        kind: knd,
        loc,
        lang: Lang::References,
    }
}
