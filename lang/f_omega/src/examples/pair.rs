use crate::{terms::Term, types::Type};
use common::{
    kinds::Kind,
    terms::{App, Lambda, Pack, Record, TyApp, TyLambda, Variable},
    types::{Exists, Forall, Fun, OpApp, OpLambda, Record as RecordTy, TypeVariable},
    TypeVar, Var,
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
                        Fun::new(
                            TypeVariable::new("X"),
                            Fun::new(
                                TypeVariable::new("Y"),
                                OpApp::new(
                                    OpApp::new(TypeVariable::new("Pair"), TypeVariable::new("X")),
                                    TypeVariable::new("Y"),
                                ),
                            ),
                        ),
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
                        Fun::new(
                            OpApp::new(
                                OpApp::new(TypeVariable::new("Pair"), TypeVariable::new("X")),
                                TypeVariable::new("Y"),
                            ),
                            TypeVariable::new("X"),
                        ),
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
                        Fun::new(
                            OpApp::new(
                                OpApp::new(TypeVariable::new("Pair"), TypeVariable::new("X")),
                                TypeVariable::new("Y"),
                            ),
                            TypeVariable::new("Y"),
                        ),
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
                    Fun::new(
                        Fun::new(
                            TypeVariable::new("X"),
                            Fun::new(TypeVariable::new("Y"), TypeVariable::new("R")),
                        ),
                        TypeVariable::new("R"),
                    ),
                ),
            ),
        ),
        Record::new(HashMap::<Var, Term>::from([
            (
                "pair".to_owned(),
                TyLambda::new(
                    "X",
                    Kind::Star,
                    TyLambda::new(
                        "Y",
                        Kind::Star,
                        Lambda::new(
                            "x",
                            TypeVariable::new("X"),
                            Lambda::new(
                                "y",
                                TypeVariable::new("Y"),
                                TyLambda::new(
                                    "R",
                                    Kind::Star,
                                    Lambda::new(
                                        "p",
                                        Fun::new(
                                            TypeVariable::new("X"),
                                            Fun::new(
                                                TypeVariable::new("Y"),
                                                TypeVariable::new("R"),
                                            ),
                                        ),
                                        App::new(
                                            App::new(Variable::new("p"), Variable::new("x")),
                                            Variable::new("y"),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                )
                .into(),
            ),
            (
                "fst".to_owned(),
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
                                Fun::new(
                                    Fun::new(
                                        TypeVariable::new("X"),
                                        Fun::new(TypeVariable::new("Y"), TypeVariable::new("R")),
                                    ),
                                    TypeVariable::new("R"),
                                ),
                            ),
                            App::new(
                                TyApp::new(Variable::new("p"), TypeVariable::new("X")),
                                Lambda::new(
                                    "x",
                                    TypeVariable::new("X"),
                                    Lambda::new("y", TypeVariable::new("Y"), Variable::new("x")),
                                ),
                            ),
                        ),
                    ),
                )
                .into(),
            ),
            (
                "snd".to_owned(),
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
                                Fun::new(
                                    Fun::new(
                                        TypeVariable::new("X"),
                                        Fun::new(TypeVariable::new("Y"), TypeVariable::new("R")),
                                    ),
                                    TypeVariable::new("R"),
                                ),
                            ),
                            App::new(
                                TyApp::new(Variable::new("p"), TypeVariable::new("Y")),
                                Lambda::new(
                                    "x",
                                    TypeVariable::new("X"),
                                    Lambda::new("y", TypeVariable::new("Y"), Variable::new("y")),
                                ),
                            ),
                        ),
                    ),
                )
                .into(),
            ),
        ])),
        pair_sig(),
    )
    .into()
}

#[cfg(test)]
mod pair_tests {
    use super::{pair_adt, pair_sig};
    use common::{
        check::{Kindcheck, Typecheck},
        kinds::Kind,
    };

    #[test]
    fn check_sig() {
        let result = pair_sig().check_kind(&mut Default::default()).unwrap();
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
