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
    use super::{fold, leaf, node, Term};

    #[test]
    fn test_leaf() {
        let result = leaf();
        let expected = Term::Lambda(
            "v".to_owned(),
            Box::new(Term::Lambda(
                "f".to_owned(),
                Box::new(Term::Lambda(
                    "g".to_owned(),
                    Box::new(Term::App(
                        Box::new(Term::Var("g".to_owned())),
                        Box::new(Term::Var("v".to_owned())),
                    )),
                )),
            )),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn test_node() {
        let result = node();
        let expected = Term::Lambda(
            "v".to_owned(),
            Box::new(Term::Lambda(
                "l".to_owned(),
                Box::new(Term::Lambda(
                    "r".to_owned(),
                    Box::new(Term::Lambda(
                        "f".to_owned(),
                        Box::new(Term::Lambda(
                            "g".to_owned(),
                            Box::new(Term::App(
                                Box::new(Term::App(
                                    Box::new(Term::App(
                                        Box::new(Term::Var("f".to_owned())),
                                        Box::new(Term::Var("v".to_owned())),
                                    )),
                                    Box::new(Term::App(
                                        Box::new(Term::App(
                                            Box::new(Term::Var("l".to_owned())),
                                            Box::new(Term::Var("f".to_owned())),
                                        )),
                                        Box::new(Term::Var("g".to_owned())),
                                    )),
                                )),
                                Box::new(Term::App(
                                    Box::new(Term::App(
                                        Box::new(Term::Var("r".to_owned())),
                                        Box::new(Term::Var("f".to_owned())),
                                    )),
                                    Box::new(Term::Var("g".to_owned())),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn test_fold() {
        let result = fold();
        let expected = Term::Lambda(
            "f".to_owned(),
            Box::new(Term::Lambda(
                "g".to_owned(),
                Box::new(Term::Lambda(
                    "t".to_owned(),
                    Box::new(Term::App(
                        Box::new(Term::App(
                            Box::new(Term::Var("t".to_owned())),
                            Box::new(Term::Var("f".to_owned())),
                        )),
                        Box::new(Term::Var("g".to_owned())),
                    )),
                )),
            )),
        );
        assert_eq!(result, expected)
    }
}
