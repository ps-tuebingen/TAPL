use super::object;
use crate::syntax::{
    kinds::Kind,
    terms::{App, Lambda, Pack, Record, RecordProj, Succ, Term, TyLambda, Unpack},
    types::{Fun, OpApp, OpLambda, RecordTy, Type},
};

pub fn counter_m() -> Type {
    OpLambda::new(
        "R",
        Kind::Star,
        RecordTy::new(vec![
            ("get", Fun::new("R", Type::Nat).into()),
            ("inc", Fun::new("R", "R").into()),
        ]),
    )
    .into()
}

pub fn counter() -> Type {
    OpApp::new(object(), counter_m()).into()
}

pub fn send_inc() -> Term {
    TyLambda::new(
        "M",
        counter_m(),
        Lambda::new(
            "c",
            OpApp::new(object(), "M"),
            Unpack::new(
                "X",
                "b",
                "c",
                Pack::new(
                    "X",
                    Record::new(vec![
                        (
                            "state",
                            App::new(
                                RecordProj::new(RecordProj::new("b", "methods"), "inc"),
                                RecordProj::new("b", "state"),
                            )
                            .into(),
                        ),
                        ("methods", RecordProj::new("b", "methods").into()),
                    ]),
                    OpApp::new(object(), "M"),
                ),
            ),
        ),
    )
    .into()
}

pub fn send_get() -> Term {
    TyLambda::new(
        "M",
        counter_m(),
        Lambda::new(
            "c",
            OpApp::new(object(), "M"),
            Unpack::new(
                "X",
                "b",
                "c",
                App::new(RecordProj::new(RecordProj::new("b", "methods"), "get"), "c"),
            ),
        ),
    )
    .into()
}

pub fn counter_r() -> Type {
    RecordTy::new(vec![("x", Type::Nat)]).into()
}

pub fn counter_class() -> Term {
    Record::new(vec![
        (
            "get",
            Lambda::new("r", counter_r(), RecordProj::new("r", "x")).into(),
        ),
        (
            "inc",
            Lambda::new(
                "r",
                counter_r(),
                Record::new(vec![("x", Succ::new(RecordProj::new("r", "x")).into())]),
            )
            .into(),
        ),
    ])
    .into()
}

pub fn new_counter() -> Term {
    Pack::new(
        counter_r(),
        Record::new(vec![
            ("state", Record::new(vec![("x", Term::Zero)]).into()),
            ("methods", counter_class()),
        ]),
        counter(),
    )
    .into()
}

#[cfg(test)]
mod counter_tests {
    use super::{
        counter, counter_class, counter_m, counter_r, new_counter, object, send_get, send_inc,
    };
    use crate::{
        check::Check,
        eval::Eval,
        syntax::types::{Fun, OpApp, Type, Universal},
    };

    #[test]
    fn check_send_inc() {
        let result = send_inc().check(&mut Default::default()).unwrap();
        let expected = Universal::new(
            "M",
            counter_m(),
            Fun::new(OpApp::new(object(), "M"), OpApp::new(object(), "M")),
        )
        .into();
        result.check_equal(&expected).unwrap();
    }

    #[test]
    fn check_send_get() {
        let result = send_get().check(&mut Default::default()).unwrap();
        let expected = <Universal as Into<Type>>::into(Universal::new(
            "M",
            counter_m(),
            Fun::new(OpApp::new(object(), "M"), Type::Nat),
        ))
        .eval(&mut Default::default())
        .unwrap();
        result.check_equal(&expected).unwrap();
    }

    #[test]
    fn check_class() {
        let result = counter_class().check(&mut Default::default()).unwrap();
        let expected = <OpApp as Into<Type>>::into(OpApp::new(counter_m(), counter_r()))
            .eval(&mut Default::default())
            .unwrap();
        result.check_equal(&expected).unwrap();
    }

    #[test]
    fn check_new() {
        let result = new_counter().check(&mut Default::default()).unwrap();
        let expected = counter();
        result.check_equal(&expected).unwrap();
    }
}
