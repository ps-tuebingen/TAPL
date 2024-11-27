pub mod bool;
pub mod list;
pub mod num;
pub mod pair;
pub mod tree;

use crate::{parse::parse, terms::Term};

pub fn id() -> Term {
    parse(&mut "\\x.x".to_owned()).unwrap()
}

pub fn omega() -> Term {
    parse(&mut "(\\x. x x) (\\x. x x)".to_owned()).unwrap()
}

#[cfg(test)]
mod examples_tests {
    use super::{id, omega};

    #[test]
    fn test_terms() {
        id();
        omega();
    }
}
