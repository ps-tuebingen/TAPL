pub mod num;
pub mod tree;

use crate::{parse::parse, terms::Term};

pub fn id() -> Term {
    parse(&mut "\\x.x".to_owned()).unwrap()
}
