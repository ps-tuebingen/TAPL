use crate::{
    terms::{App, IsZero, Lambda, Pack, Record, RecordProj, Succ, Term, Unpack, Zero},
    types::Type,
};

pub fn counter_adt() -> Term {
    Pack::new(
        Type::Nat,
        Record::new(vec![
            ("new", Succ::new(Zero).into()),
            ("get", Lambda::new("i", Type::Nat, "i").into()),
            ("inc", Lambda::new("i", Type::Nat, Succ::new("i")).into()),
        ]),
        Type::pack(
            "Counter",
            Type::record(vec![
                ("new", "Counter".into()),
                ("get", Type::fun("Counter".into(), Type::Nat)),
                ("inc", Type::fun("Counter".into(), "Counter".into())),
            ]),
        ),
    )
    .into()
}

pub fn counter_adt_rec() -> Term {
    Pack::new(
        Type::record(vec![("x", Type::Nat)]),
        Record::new(vec![
            (
                "new",
                Record::new(vec![("x", Succ::new(Zero).into())]).into(),
            ),
            (
                "get",
                Lambda::new(
                    "i",
                    Type::record(vec![("x", Type::Nat)]),
                    RecordProj::new("i", "x"),
                )
                .into(),
            ),
            (
                "inc",
                Lambda::new(
                    "i",
                    Type::record(vec![("x", Type::Nat)]),
                    Record::new(vec![("x", Succ::new(RecordProj::new("i", "x")).into())]),
                )
                .into(),
            ),
        ]),
        Type::pack(
            "Counter",
            Type::record(vec![
                ("new", "Counter".into()),
                ("get", Type::fun("Counter".into(), Type::Nat)),
                ("inc", Type::fun("Counter".into(), "Counter".into())),
            ]),
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
                "Counter".into(),
                Record::new(vec![
                    ("new", RecordProj::new("counter", "new").into()),
                    (
                        "read",
                        Lambda::new(
                            "c",
                            "Counter".into(),
                            IsZero::new(App::new(RecordProj::new("counter", "get"), "c")),
                        )
                        .into(),
                    ),
                    (
                        "toggle",
                        Lambda::new(
                            "c",
                            "Counter".into(),
                            App::new(RecordProj::new("counter", "inc"), "c"),
                        )
                        .into(),
                    ),
                    (
                        "reset",
                        Lambda::new("c", "Counter".into(), RecordProj::new("counter", "new"))
                            .into(),
                    ),
                ]),
                Type::pack(
                    "FlipFlop",
                    Type::record(vec![
                        ("new", "FlipFlop".into()),
                        ("read", Type::fun("FlipFlop".into(), Type::Bool)),
                        ("toggle", Type::fun("FlipFlop".into(), "FlipFlop".into())),
                        ("reset", Type::fun("FlipFlop".into(), "FlipFlop".into())),
                    ]),
                ),
            ),
            App::new(
                RecordProj::new("flipflop", "read"),
                App::new(
                    RecordProj::new("flipflop", "toggle"),
                    App::new(
                        RecordProj::new("flipflop", "toggle"),
                        RecordProj::new("flipflop", "new"),
                    ),
                ),
            ),
        ),
    )
    .into()
}

#[cfg(test)]
mod adt_tests {
    use super::{counter_adt, counter_adt_rec, flip_flop};
    use crate::{check::Check, types::Type};

    #[test]
    fn check_counter() {
        let result = counter_adt().check(&mut Default::default()).unwrap();
        let expected = Type::pack(
            "Counter",
            Type::record(vec![
                ("new", "Counter".into()),
                ("get", Type::fun("Counter".into(), Type::Nat)),
                ("inc", Type::fun("Counter".into(), "Counter".into())),
            ]),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn check_counter_rec() {
        let result = counter_adt_rec().check(&mut Default::default()).unwrap();
        let expected = Type::pack(
            "Counter",
            Type::record(vec![
                ("new", "Counter".into()),
                ("get", Type::fun("Counter".into(), Type::Nat)),
                ("inc", Type::fun("Counter".into(), "Counter".into())),
            ]),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn check_flipflop() {
        let result = flip_flop().check(&mut Default::default()).unwrap();
        let expected = Type::Bool;
        assert_eq!(result, expected)
    }
}
