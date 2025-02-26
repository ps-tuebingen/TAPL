use crate::{parse::parse, terms::Term};

pub fn leaf() -> Term {
    parse(&mut "\\v.\\f.\\g.g v".to_owned()).unwrap()
}

pub fn node() -> Term {
    parse(&mut "\\v.\\l.\\r.\\f.\\g.(((f v) ((l f) g)) ((r f) g))".to_owned()).unwrap()
}

pub fn fold() -> Term {
    parse(&mut "\\f.\\g.\\t. (t f) g".to_owned()).unwrap()
}

#[cfg(test)]
mod tree_tests {
    use super::{fold, leaf, node};

    #[test]
    fn test_terms() {
        leaf();
        node();
        fold();
    }
}
