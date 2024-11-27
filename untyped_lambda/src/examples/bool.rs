use crate::{parse::parse, terms::Term};

pub fn tru() -> Term {
    parse(&mut "\\t.\\f.t".to_owned()).unwrap()
}

pub fn fls() -> Term {
    parse(&mut "\\t.\\f.f".to_owned()).unwrap()
}

pub fn ifthenelse() -> Term {
    parse(&mut "\\l.\\m.\\n. (l m) n".to_owned()).unwrap()
}

pub fn and() -> Term {
    parse(&mut format!("\\b.\\c. (b c) ({})", fls())).unwrap()
}

pub fn or() -> Term {
    parse(&mut format!("\\b.\\c. (b ({})) c", tru())).unwrap()
}

pub fn not() -> Term {
    parse(&mut format!("\\b. (b ({})) ({})", fls(), tru())).unwrap()
}

#[cfg(test)]
mod bool_tests {
    use super::{and, fls, ifthenelse, not, or, tru};

    #[test]
    fn test_terms() {
        tru();
        fls();
        ifthenelse();
        and();
        not();
        or();
    }
}
