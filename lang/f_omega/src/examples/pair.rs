use crate::terms::Term;
use common::{
    kinds::Kind,
    terms::{App, Lambda, Pack, Record, TyApp, TyLambda},
    types::{Exists, Forall, Fun, OpApp, OpLambda, Record as RecordTy, Type},
    TypeVar,
};
use std::collections::HashMap;

pub fn pair_sig() -> Type {
    Exists::new(
        "Pair",
        Kind::Star.abs().abs(),
        RecordTy::new(HashMap::<TypeVar, Type>::from([
            (
                "pair".to_owned(),
                Forall::new(
                    "X",
                    Kind::Star,
                    Forall::new(
                        "Y",
                        Kind::Star,
                        Fun::new("X", Fun::new("Y", OpApp::new(OpApp::new("Pair", "X"), "Y"))),
                    ),
                )
                .into(),
            ),
            (
                "fst".to_owned(),
                Forall::new(
                    "X",
                    Kind::Star,
                    Forall::new(
                        "Y",
                        Kind::Star,
                        Fun::new(OpApp::new(OpApp::new("Pair", "X"), "Y"), "X"),
                    ),
                )
                .into(),
            ),
            (
                "snd".to_owned(),
                Forall::new(
                    "X",
                    Kind::Star,
                    Forall::new(
                        "Y",
                        Kind::Star,
                        Fun::new(OpApp::new(OpApp::new("Pair", "X"), "Y"), "Y"),
                    ),
                )
                .into(),
            ),
        ])),
    )
    .into()
}

pub fn pair_adt() -> Term {
    Pack::new(
        OpLambda::new(
            "X",
            Kind::Star,
            OpLambda::new(
                "Y",
                Kind::Star,
                Forall::new(
                    "R",
                    Kind::Star,
                    Fun::new(Fun::new("X", Fun::new("Y", "R")), "R"),
                ),
            ),
        ),
        Record::new(vec![
            (
                "pair",
                TyLambda::new(
                    "X",
                    Kind::Star,
                    TyLambda::new(
                        "Y",
                        Kind::Star,
                        Lambda::new(
                            "x",
                            "X",
                            Lambda::new(
                                "y",
                                "Y",
                                TyLambda::new(
                                    "R",
                                    Kind::Star,
                                    Lambda::new(
                                        "p",
                                        Fun::new("X", Fun::new("Y", "R")),
                                        App::new(App::new("p", "x"), "y"),
                                    ),
                                ),
                            ),
                        ),
                    ),
                )
                .into(),
            ),
            (
                "fst",
                TyLambda::new(
                    "X",
                    Kind::Star,
                    TyLambda::new(
                        "Y",
                        Kind::Star,
                        Lambda::new(
                            "p",
                            Forall::new(
                                "R",
                                Kind::Star,
                                Fun::new(Fun::new("X", Fun::new("Y", "R")), "R"),
                            ),
                            App::new(
                                TyApp::new("p", "X"),
                                Lambda::new("x", "X", Lambda::new("y", "Y", "x")),
                            ),
                        ),
                    ),
                )
                .into(),
            ),
            (
                "snd",
                TyLambda::new(
                    "X",
                    Kind::Star,
                    TyLambda::new(
                        "Y",
                        Kind::Star,
                        Lambda::new(
                            "p",
                            Forall::new(
                                "R",
                                Kind::Star,
                                Fun::new(Fun::new("X", Fun::new("Y", "R")), "R"),
                            ),
                            App::new(
                                TyApp::new("p", "Y"),
                                Lambda::new("x", "X", Lambda::new("y", "Y", "y")),
                            ),
                        ),
                    ),
                )
                .into(),
            ),
        ]),
        pair_sig(),
    )
    .into()
}

#[cfg(test)]
mod pair_tests {
    use super::{pair_adt, pair_sig};
    use crate::syntax::kinds::Kind;
    use common::Typecheck;

    #[test]
    fn check_sig() {
        let result = pair_sig().check(&mut Default::default()).unwrap();
        let expected = Kind::Star;
        assert_eq!(result, expected)
    }

    #[test]
    fn check_pair() {
        let result = pair_adt()
            .check(&mut Default::default())
            .map_err(|err| err.to_string())
            .unwrap();
        let expected = pair_sig();
        assert_eq!(result, expected)
    }
}
