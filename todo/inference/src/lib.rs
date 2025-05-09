pub mod bidirectional;
pub mod constraints;
pub mod parser;
pub mod syntax;
pub mod types;

use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    langs::Lang,
};

pub fn to_err(knd: ErrorKind, loc: ErrorLocation) -> Error {
    Error {
        kind: knd,
        loc,
        lang: Lang::Inference,
    }
}
