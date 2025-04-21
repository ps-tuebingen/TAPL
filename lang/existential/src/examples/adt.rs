use crate::{terms::Term, types::Type};
use common::{
    kinds::Kind,
    terms::{App, IsZero, Lambda, Num, Pack, Record, RecordProj, Succ, Unpack, Variable},
    types::{Bool, Exists, Fun, Nat, Record as RecordTy, TypeVariable},
    TypeVar, Var,
};
use std::collections::HashMap;

pub fn counter_adt() -> Term {
    Pack::new(
        Nat::new(),
        Record::new(HashMap::<Var, Term>::from([
            ("new".to_owned(), Succ::new(Num::new(0)).into()),
            (
                "get".to_owned(),
                Lambda::new("i", Nat::new(), Variable::new("i")).into(),
            ),
            (
                "inc".to_owned(),
                Lambda::new("i", Nat::new(), Succ::new(Variable::new("i"))).into(),
            ),
        ])),
        Exists::new(
            "Counter",
            Kind::Star,
            RecordTy::new(HashMap::<TypeVar, Type>::from([
                ("new".to_owned(), TypeVariable::new("Counter").into()),
                (
                    "get".to_owned(),
                    Fun::new(TypeVariable::new("Counter"), Nat::new()).into(),
                )
                    .into(),
                (
                    "inc".to_owned(),
                    Fun::new(TypeVariable::new("Counter"), TypeVariable::new("Counter")).into(),
                ),
            ])),
        ),
    )
    .into()
}

pub fn counter_adt_rec() -> Term {
    Pack::new(
        RecordTy::new(HashMap::<TypeVar, Type>::from([(
            "x".to_owned(),
            Nat::new().into(),
        )])),
        Record::new(HashMap::<Var, Term>::from([
            (
                "new".to_owned(),
                Record::new(HashMap::<Var, Term>::from([(
                    "x".to_owned(),
                    Succ::new(Num::new(0)).into(),
                )]))
                .into(),
            ),
            (
                "get".to_owned(),
                Lambda::new(
                    "i",
                    RecordTy::new(HashMap::<TypeVar, Type>::from([(
                        "x".to_owned(),
                        Nat::new().into(),
                    )])),
                    RecordProj::new(Variable::new("i"), "x"),
                )
                .into(),
            ),
            (
                "inc".to_owned(),
                Lambda::new(
                    "i",
                    RecordTy::new(HashMap::<TypeVar, Type>::from([(
                        "x".to_owned(),
                        Nat::new().into(),
                    )])),
                    Record::new(HashMap::<Var, Term>::from([(
                        "x".to_owned(),
                        Succ::new(RecordProj::new(Variable::new("i"), "x")).into(),
                    )])),
                )
                .into(),
            ),
        ])),
        Exists::new(
            "Counter",
            Kind::Star,
            RecordTy::new(HashMap::<Var, Type>::from([
                ("new".to_owned(), TypeVariable::new("Counter").into()),
                (
                    "get".to_owned(),
                    Fun::new(TypeVariable::new("Counter"), Nat::new()).into(),
                ),
                (
                    "inc".to_owned(),
                    Fun::new(TypeVariable::new("Counter"), TypeVariable::new("Counter")).into(),
                ),
            ])),
        ),
    )
    .into()
}

pub fn flip_flop() -> Term {
    Unpack::new(
        "Counter",
        "counter",
        counter_adt(),
        Unpack::new(
            "FlipFlop",
            "flipflop",
            Pack::new(
                TypeVariable::new("Counter"),
                Record::new(HashMap::<Var, Term>::from([
                    (
                        "new".to_owned(),
                        RecordProj::new(Variable::new("counter"), "new").into(),
                    ),
                    (
                        "read".to_owned(),
                        Lambda::new(
                            "c",
                            TypeVariable::new("Counter"),
                            IsZero::new(App::new(
                                RecordProj::new(Variable::new("counter"), "get"),
                                Variable::new("c"),
                            )),
                        )
                        .into(),
                    ),
                    (
                        "toggle".to_owned(),
                        Lambda::new(
                            "c",
                            TypeVariable::new("Counter"),
                            App::new(
                                RecordProj::new(Variable::new("counter"), "inc"),
                                Variable::new("c"),
                            ),
                        )
                        .into(),
                    ),
                    (
                        "reset".to_owned(),
                        Lambda::new(
                            "c",
                            TypeVariable::new("Counter"),
                            RecordProj::new(Variable::new("counter"), "new"),
                        )
                        .into(),
                    ),
                ])),
                Exists::new(
                    "FlipFlop",
                    Kind::Star,
                    RecordTy::new(HashMap::<TypeVar, Type>::from([
                        ("new".to_owned(), TypeVariable::new("FlipFlop").into()),
                        (
                            "read".to_owned(),
                            Fun::new(TypeVariable::new("FlipFlop"), Bool::new()).into(),
                        ),
                        (
                            "toggle".to_owned(),
                            Fun::new(TypeVariable::new("FlipFlop"), TypeVariable::new("FlipFlop"))
                                .into(),
                        ),
                        (
                            "reset".to_owned(),
                            Fun::new(TypeVariable::new("FlipFlop"), TypeVariable::new("FlipFlop"))
                                .into(),
                        ),
                    ])),
                ),
            ),
            App::new(
                RecordProj::new(Variable::new("flipflop"), "read"),
                App::new(
                    RecordProj::new(Variable::new("flipflop"), "toggle"),
                    App::new(
                        RecordProj::new(Variable::new("flipflop"), "toggle"),
                        RecordProj::new(Variable::new("flipflop"), "new"),
                    ),
                ),
            ),
        ),
    )
    .into()
}

#[cfg(test)]
mod adt_tests {
    use super::{counter_adt, counter_adt_rec, flip_flop, Type};
    use common::{
        check::Typecheck,
        kinds::Kind,
        types::{Bool, Exists, Fun, Nat, Record as RecordTy, TypeVariable},
        TypeVar,
    };
    use std::collections::HashMap;

    #[test]
    fn check_counter() {
        let result = counter_adt().check(&mut Default::default()).unwrap();
        let expected = Exists::new(
            "Counter",
            Kind::Star,
            RecordTy::new(HashMap::<TypeVar, Type>::from([
                ("new".to_owned(), TypeVariable::new("Counter").into()),
                (
                    "get".to_owned(),
                    Fun::new(TypeVariable::new("Counter"), Nat::new()).into(),
                ),
                (
                    "inc".to_owned(),
                    Fun::new(TypeVariable::new("Counter"), TypeVariable::new("Counter")).into(),
                ),
            ])),
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_counter_rec() {
        let result = counter_adt_rec().check(&mut Default::default()).unwrap();
        let expected = Exists::new(
            "Counter",
            RecordTy::new(HashMap::<TypeVar, Type>::from([
                ("new".to_owned(), TypeVariable::new("Counter").into()),
                (
                    "get".to_owned(),
                    Fun::new(TypeVariable::new("Counter"), Nat::new()).into(),
                ),
                (
                    "inc".to_owned(),
                    Fun::new(TypeVariable::new("Counter"), TypeVariable::new("Counter")).into(),
                ),
            ])),
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_flipflop() {
        let result = flip_flop().check(&mut Default::default()).unwrap();
        let expected = Bool.into();
        assert_eq!(result, expected)
    }
}
