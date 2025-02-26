use super::{
    bool::{fls, tru},
    pair::{fst, pair, snd},
};
use crate::{parse::parse, terms::Term};

pub fn c0() -> Term {
    parse(&mut "\\s.\\z.z".to_owned()).unwrap()
}

pub fn c1() -> Term {
    parse(&mut "\\s.\\z. s z".to_owned()).unwrap()
}

pub fn cn(n: usize) -> Term {
    let mut s = "\\s.\\z.".to_owned();
    s.push_str(&cn_inner(n));
    parse(&mut s).unwrap()
}

fn cn_inner(n: usize) -> String {
    if n == 0 {
        "z".to_owned()
    } else {
        let inner_prev = cn_inner(n - 1);
        format!("s ({inner_prev})")
    }
}

pub fn succ() -> Term {
    parse(&mut "\\n.\\s.\\z.s ((n s) z)".to_owned()).unwrap()
}

pub fn plus() -> Term {
    parse(&mut "\\m.\\n.\\s.\\z. ((m s) ((n s) z))".to_owned()).unwrap()
}

pub fn times() -> Term {
    parse(&mut format!("\\m.\\n. (m (({}) n)) ({})", plus(), c0())).unwrap()
}

pub fn pow() -> Term {
    parse(&mut format!("\\m.\\n.(m (({}) n)) ({})", times(), c0())).unwrap()
}

pub fn iszero() -> Term {
    parse(&mut format!("\\m. (m (\\x.({}))) ({})", fls(), tru())).unwrap()
}

pub fn prd() -> Term {
    let zz = format!("((({}) ({})) ({}))", pair(), c0(), c0());
    let ss = format!(
        "(\\p. (({}) (({}) p)) ((({}) ({})) (({}) p)) )",
        pair(),
        snd(),
        plus(),
        c1(),
        snd()
    );

    parse(&mut format!("\\m. ({}) ((m ({})) ({}))", fst(), ss, zz)).unwrap()
}

pub fn sub() -> Term {
    parse(&mut format!("\\m.\\n. (m (({}) n)) ({})", prd(), c0())).unwrap()
}

pub fn equal() -> Term {
    parse(&mut format!("\\m.\\n. ({}) ((({}) m) n)", iszero(), sub())).unwrap()
}

#[cfg(test)]
mod num_tests {
    use super::{c0, c1, cn, equal, iszero, plus, pow, prd, sub, succ, times};

    #[test]
    fn test_terms() {
        succ();
        plus();
        times();
        pow();
        iszero();
        prd();
        sub();
        equal();
    }

    #[test]
    fn test_zero() {
        let result = cn(0);
        let expected = c0();
        assert_eq!(result, expected)
    }

    #[test]
    fn test_one() {
        let result = cn(1);
        let expected = c1();
        assert_eq!(result, expected)
    }
}
