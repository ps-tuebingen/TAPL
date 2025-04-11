use common::{
    errors::{Error, ErrorKind, ErrorLocation},
    langs::Lang,
};
pub mod check;
pub mod eval;
pub mod examples;
pub mod parser;
pub mod terms;
pub mod traits;
pub mod types;

pub fn to_err(knd: ErrorKind, loc: ErrorLocation) -> Error {
    Error {
        kind: knd,
        loc,
        lang: Lang::Existential,
    }
}
