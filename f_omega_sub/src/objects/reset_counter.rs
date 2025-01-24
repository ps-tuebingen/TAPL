use super::{
    counter::{counter_class, counter_r},
    object,
};
use crate::syntax::{
    kinds::Kind,
    terms::{App, Lambda, Let, Pack, Record, RecordProj, Term, TyLambda, Unpack},
    types::{Fun, OpApp, OpLambda, RecordTy, Type},
};

pub fn reset_counter_m() -> Type {
    OpLambda::new(
        "R",
        Kind::Star,
        RecordTy::new(vec![
            ("get", Fun::new("R", Type::Nat).into()),
            ("inc", Fun::new("R", "R").into()),
            ("reset", Fun::new("R", "R").into()),
        ]),
    )
    .into()
}
pub fn reset_counter() -> Type {
    OpApp::new(object(), reset_counter_m()).into()
}

pub fn reset_counter_class() -> Term {
    Let::new(
        "super",
        counter_class(),
        Record::new(vec![
            ("get", RecordProj::new("super", "get").into()),
            ("inc", RecordProj::new("super", "inc").into()),
            (
                "reset",
                Lambda::new("r", counter_r(), Record::new(vec![("x", Term::Zero)])).into(),
            ),
        ]),
    )
    .into()
}

pub fn send_reset() -> Term {
    TyLambda::new(
        "M",
        reset_counter_m(),
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
                                RecordProj::new(RecordProj::new("b", "methods"), "reset"),
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

#[cfg(test)]
mod reset_counter_tests {
    use super::{object, reset_counter, reset_counter_class, reset_counter_m, send_reset};
    use crate::{
        check::{check_subtype, Check},
        eval::Eval,
        objects::counter::{counter, counter_m, counter_r},
        syntax::types::{Fun, OpApp, Type, Universal},
    };

    #[test]
    fn reset_counter_sub() {
        let result = check_subtype(&reset_counter(), &counter(), &mut Default::default());
        assert!(result.is_ok())
    }

    #[test]
    fn reset_counter_m_sub() {
        check_subtype(&reset_counter_m(), &counter_m(), &mut Default::default()).unwrap();
    }

    #[test]
    fn check_class() {
        let result = reset_counter_class()
            .check(&mut Default::default())
            .unwrap();
        let expected = <OpApp as Into<Type>>::into(OpApp::new(reset_counter_m(), counter_r()))
            .eval(&mut Default::default())
            .unwrap();
        result.check_equal(&expected).unwrap();
    }
    #[test]
    fn check_send_reset() {
        let result = send_reset().check(&mut Default::default()).unwrap();
        let expected = Universal::new(
            "M",
            reset_counter_m(),
            Fun::new(OpApp::new(object(), "M"), OpApp::new(object(), "M")),
        )
        .into();
        result.check_equal(&expected).unwrap();
    }
}
