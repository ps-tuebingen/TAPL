use super::bool::{fls, tru};
use crate::{parse::parse, terms::Term};

pub fn pair() -> Term {
    parse(&mut "\\f.\\s.\\b. (b f) s".to_owned()).unwrap()
}

pub fn fst() -> Term {
    parse(&mut format!("\\p.p ({})", tru())).unwrap()
}

pub fn snd() -> Term {
    parse(&mut format!("\\p.p ({})", fls())).unwrap()
}

#[cfg(test)]
mod pair_tests {
    use super::{fst, pair, snd};

    #[test]
    fn test_terms() {
        pair();
        fst();
        snd();
    }
}
