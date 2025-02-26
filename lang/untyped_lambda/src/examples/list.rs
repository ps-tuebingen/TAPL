use super::{
    bool::{fls, tru},
    pair::{fst, pair, snd},
};
use crate::{parse::parse, terms::Term};

pub fn nil() -> Term {
    parse(&mut format!("(({}) ({})) ({}) ", pair(), tru(), tru())).unwrap()
}

pub fn isnil() -> Term {
    fst()
}

pub fn cons() -> Term {
    parse(&mut format!(
        "\\h.\\t. (({}) ({})) ((({}) h) t)",
        pair(),
        fls(),
        pair()
    ))
    .unwrap()
}

pub fn head() -> Term {
    parse(&mut format!("\\z.({}) (({}) z)", fst(), snd())).unwrap()
}

pub fn tail() -> Term {
    parse(&mut format!("\\z.({}) (({}) z)", snd(), snd())).unwrap()
}

#[cfg(test)]
mod list_tests {
    use super::{cons, head, isnil, nil, tail};

    #[test]
    fn test_terms() {
        nil();
        cons();
        isnil();
        head();
        tail();
    }
}
