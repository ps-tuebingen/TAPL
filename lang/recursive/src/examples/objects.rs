use crate::{
    terms::{App, Fix, Fold, Lambda, Let, Pred, Record, RecordProj, Succ, Term, Zero},
    types::Type,
};
use std::collections::HashMap;

pub fn ty_counter() -> Type {
    Type::mu(
        "C",
        Type::Record(HashMap::from([
            ("get".to_owned(), Type::Nat),
            ("inc".to_owned(), Type::fun(Type::Unit, "C".into())),
            ("dec".to_owned(), Type::fun(Type::Unit, "C".into())),
        ])),
    )
}

pub fn new_counter() -> Term {
    Let::new(
        "create",
        Fix::new(
            Lambda::new(
                "f",
                Type::fun(
                    Type::Record(HashMap::from([("x".to_owned(), Type::Nat)])),
                    ty_counter(),
                ),
                Lambda::new(
                    "s",
                    Type::Record(HashMap::from([("x".to_owned(), Type::Nat)])),
                    Fold::new(
                        Record::new(&[
                            (
                                "get",
                                RecordProj::new(
                                    "s".into(),
                                    Type::Record(HashMap::from([("x".to_owned(), Type::Nat)])),
                                    "x",
                                )
                                .into(),
                            ),
                            (
                                "inc",
                                Lambda::new(
                                    "_",
                                    Type::Unit,
                                    App::new(
                                        "f".into(),
                                        Record::new(&[(
                                            "x",
                                            Succ::new(
                                                RecordProj::new(
                                                    "s".into(),
                                                    Type::Record(HashMap::from([(
                                                        "x".to_owned(),
                                                        Type::Nat,
                                                    )])),
                                                    "x",
                                                )
                                                .into(),
                                            )
                                            .into(),
                                        )])
                                        .into(),
                                    )
                                    .into(),
                                )
                                .into(),
                            ),
                            (
                                "dec",
                                Lambda::new(
                                    "_",
                                    Type::Unit,
                                    App::new(
                                        "f".into(),
                                        Record::new(&[(
                                            "x",
                                            Pred::new(
                                                RecordProj::new(
                                                    "s".into(),
                                                    Type::Record(HashMap::from([(
                                                        "x".to_owned(),
                                                        Type::Nat,
                                                    )])),
                                                    "x",
                                                )
                                                .into(),
                                            )
                                            .into(),
                                        )])
                                        .into(),
                                    )
                                    .into(),
                                )
                                .into(),
                            ),
                        ])
                        .into(),
                        ty_counter(),
                    )
                    .into(),
                )
                .into(),
            )
            .into(),
        )
        .into(),
        App::new("create".into(), Record::new(&[("x", Zero.into())]).into()).into(),
    )
    .into()
}

#[cfg(test)]
mod counter_tests {
    use super::{new_counter, ty_counter};
    use crate::check::Check;

    #[test]
    fn check_new() {
        let result = new_counter().check(&mut Default::default()).unwrap();
        let expected = ty_counter();
        assert_eq!(result, expected)
    }
}
