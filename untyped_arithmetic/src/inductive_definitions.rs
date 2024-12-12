use super::Term;
use std::collections::HashSet;

pub fn consts(t: &Term) -> HashSet<Term> {
    match t {
        Term::True => HashSet::from([Term::True]),
        Term::False => HashSet::from([Term::False]),
        Term::If(t1, t2, t3) => {
            let mut constants = consts(t1);
            constants.extend(consts(t2));
            constants.extend(consts(t3));
            constants
        }
        Term::Zero => HashSet::from([Term::Zero]),
        Term::Succ(t) => consts(t),
        Term::Pred(t) => consts(t),
        Term::IsZero(t) => consts(t),
    }
}

pub fn size(t: &Term) -> usize {
    match t {
        Term::True => 1,
        Term::False => 1,
        Term::If(t1, t2, t3) => size(t1) + size(t2) + size(t3) + 1,
        Term::Zero => 1,
        Term::Succ(t) => size(t) + 1,
        Term::Pred(t) => size(t) + 1,
        Term::IsZero(t) => size(t) + 1,
    }
}

pub fn depth(t: &Term) -> usize {
    match t {
        Term::True => 1,
        Term::False => 1,
        Term::If(t1, t2, t3) => [depth(t1), depth(t2), depth(t3)].iter().max().unwrap() + 1,
        Term::Zero => 1,
        Term::Succ(t) => depth(t) + 1,
        Term::Pred(t) => depth(t) + 1,
        Term::IsZero(t) => depth(t) + 1,
    }
}

#[cfg(test)]
mod def_tests {
    use super::{consts, depth, size, HashSet, Term};

    fn example_simple() -> Term {
        Term::Succ(Box::new(Term::Zero))
    }

    fn example_complex() -> Term {
        Term::If(
            Box::new(Term::IsZero(Box::new(Term::Zero))),
            Box::new(Term::True),
            Box::new(Term::False),
        )
    }

    #[test]
    fn consts_simple() {
        let result = consts(&example_simple());
        let expected = HashSet::from([Term::Zero]);
        assert_eq!(result, expected)
    }

    #[test]
    fn consts_complex() {
        let result = consts(&example_complex());
        let expected = HashSet::from([Term::Zero, Term::True, Term::False]);
        assert_eq!(result, expected)
    }

    #[test]
    fn size_simple() {
        let result = size(&example_simple());
        let expected = 2;
        assert_eq!(result, expected)
    }

    #[test]
    fn size_complex() {
        let result = size(&example_complex());
        let expected = 5;
        assert_eq!(result, expected)
    }

    #[test]
    fn depth_simple() {
        let result = depth(&example_simple());
        let expected = 2;
        assert_eq!(result, expected)
    }

    #[test]
    fn depth_complex() {
        let result = depth(&example_complex());
        let expected = 3;
        assert_eq!(result, expected)
    }
}
