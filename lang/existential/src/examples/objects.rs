use crate::{
    terms::{App, Lambda, Pack, Record, RecordProj, Succ, Term, Unpack},
    types::Type,
};

pub fn ty_counter() -> Type {
    Type::pack(
        "X",
        Type::record(vec![
            ("state", "X".into()),
            (
                "methods",
                Type::record(vec![
                    ("get", Type::fun("X".into(), Type::Nat)),
                    ("inc", Type::fun("X".into(), "X".into())),
                ]),
            ),
        ]),
    )
}

pub fn counter_ex() -> Term {
    Pack::new(
        Type::Nat,
        Record::new(vec![
            ("state", 5.into()),
            (
                "methods",
                Record::new(vec![
                    ("get", Lambda::new("x", Type::Nat, "x").into()),
                    ("inc", Lambda::new("x", Type::Nat, Succ::new("x")).into()),
                ])
                .into(),
            ),
        ]),
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
            "c",
            App::new(
                RecordProj::new(RecordProj::new("body", "methods"), "get"),
                RecordProj::new("body", "state"),
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
            "c",
            Pack::new(
                "X".into(),
                Record::new(vec![
                    (
                        "state",
                        App::new(
                            RecordProj::new(RecordProj::new("body", "methods"), "inc"),
                            RecordProj::new("body", "state"),
                        )
                        .into(),
                    ),
                    ("methods", RecordProj::new("body", "methods").into()),
                ]),
                ty_counter(),
            ),
        ),
    )
    .into()
}

#[cfg(test)]
mod object_tests {
    use super::{counter_ex, send_get, send_inc, ty_counter};
    use crate::types::Type;
    use common::Typecheck;

    #[test]
    fn check_counter() {
        let result = counter_ex().check(&mut Default::default()).unwrap();
        let expected = ty_counter();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_send_get() {
        let result = send_get().check(&mut Default::default()).unwrap();
        let expected = Type::fun(ty_counter(), Type::Nat);
        assert_eq!(result, expected)
    }

    #[test]
    fn check_send_inc() {
        let result = send_inc().check(&mut Default::default()).unwrap();
        let expected = Type::fun(ty_counter(), ty_counter());
        assert_eq!(result, expected)
    }
}
