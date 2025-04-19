use crate::{terms::Term, types::Type};
use common::{
    terms::{App, Lambda, Num, Pack, Record, RecordProj, Succ, Unpack, Variable},
    types::{Exists, Fun, Nat, Record as RecordTy, TypeVariable},
    TypeVar, Var,
};
use std::collections::HashMap;

pub fn ty_counter() -> Type {
    Exists::new(
        "X",
        RecordTy::new(HashMap::<TypeVar, Type>::from([
            ("state".to_owned(), TypeVariable::new("X").into()),
            (
                "methods".to_owned(),
                RecordTy::new(HashMap::<TypeVar, Type>::from([
                    (
                        "get".to_owned(),
                        Fun::new(TypeVariable::new("X"), Nat).into(),
                    ),
                    (
                        "inc".to_owned(),
                        Fun::new(TypeVariable::new("X"), TypeVariable::new("X")).into(),
                    ),
                ]))
                .into(),
            ),
        ])),
    )
    .into()
}

pub fn counter_ex() -> Term {
    Pack::new(
        Nat,
        Record::new(HashMap::<Var, Term>::from([
            ("state".to_owned(), Num::new(5).into()),
            (
                "methods".to_owned(),
                Record::new(HashMap::<Var, Term>::from([
                    (
                        "get".to_owned(),
                        Lambda::new("x", Nat, Variable::new("x")).into(),
                    )
                        .into(),
                    (
                        "inc".to_owned(),
                        Lambda::new("x", Nat, Succ::new(Variable::new("x"))).into(),
                    ),
                ]))
                .into(),
            ),
        ])),
        ty_counter(),
    )
    .into()
}

pub fn send_get() -> Term {
    Lambda::new(
        "c",
        ty_counter(),
        Unpack::new(
            "X",
            "body",
            Variable::new("c"),
            App::new(
                RecordProj::new(RecordProj::new(Variable::new("body"), "methods"), "get"),
                RecordProj::new(Variable::new("body"), "state"),
            ),
        ),
    )
    .into()
}

pub fn send_inc() -> Term {
    Lambda::new(
        "c",
        ty_counter(),
        Unpack::new(
            "X",
            "body",
            Variable::new("c"),
            Pack::new(
                TypeVariable::new("X"),
                Record::new(HashMap::<Var, Term>::from([
                    (
                        "state".to_owned(),
                        App::new(
                            RecordProj::new(
                                RecordProj::new(Variable::new("body"), "methods"),
                                "inc",
                            ),
                            RecordProj::new(Variable::new("body"), "state"),
                        )
                        .into(),
                    ),
                    (
                        "methods".to_owned(),
                        RecordProj::new(Variable::new("body"), "methods").into(),
                    ),
                ])),
                ty_counter(),
            ),
        ),
    )
    .into()
}

#[cfg(test)]
mod object_tests {
    use super::{counter_ex, send_get, send_inc, ty_counter};
    use common::{
        check::Typecheck,
        types::{Fun, Nat},
    };

    #[test]
    fn check_counter() {
        let result = counter_ex().check(&mut Default::default()).unwrap();
        let expected = ty_counter();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_send_get() {
        let result = send_get().check(&mut Default::default()).unwrap();
        let expected = Fun::new(ty_counter(), Nat).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_send_inc() {
        let result = send_inc().check(&mut Default::default()).unwrap();
        let expected = Fun::new(ty_counter(), ty_counter()).into();
        assert_eq!(result, expected)
    }
}
