pub mod objects;
pub mod parser;
pub mod syntax;
pub mod types;
pub mod typing;

use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    langs::Lang,
};

pub fn to_err(knd: ErrorKind, loc: ErrorLocation) -> Error {
    Error {
        kind: knd,
        loc,
        lang: Lang::Subtypes,
    }
}
