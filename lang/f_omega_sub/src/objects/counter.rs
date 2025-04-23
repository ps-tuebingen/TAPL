use super::object;
use crate::{terms::Term, types::Type};
use common::{
    kinds::Kind,
    terms::{App, Lambda, LambdaSub, Num, Pack, Record, RecordProj, Succ, Unpack, Variable},
    types::{Fun, Nat, OpApp, OpLambdaSub, Record as RecordTy, Top, TypeVariable},
    Label, TypeVar, Var,
};
use std::collections::HashMap;

pub fn counter_m() -> Type {
    OpLambdaSub::new(
        "R",
        Top::new(Kind::Star),
        RecordTy::new(HashMap::<TypeVar, Type>::from([
            (
                "get".to_owned(),
                Fun::new(TypeVariable::new("R"), Nat::new()).into(),
            ),
            (
                "inc".to_owned(),
                Fun::new(TypeVariable::new("R"), TypeVariable::new("R")).into(),
            ),
        ])),
    )
    .into()
}

pub fn counter() -> Type {
    OpApp::new(object(), counter_m()).into()
}

pub fn send_inc() -> Term {
    LambdaSub::new(
        "M",
        counter_m(),
        Lambda::new(
            "c",
            OpApp::new(object(), TypeVariable::new("M")),
            Unpack::new(
                "X",
                "b",
                Variable::new("c"),
                Pack::new(
                    TypeVariable::new("X"),
                    Record::new(HashMap::<Var, Term>::from([
                        (
                            "state".to_owned(),
                            App::new(
                                RecordProj::new(
                                    RecordProj::new(Variable::new("b"), "methods"),
                                    "inc",
                                ),
                                RecordProj::new(Variable::new("b"), "state"),
                            )
                            .into(),
                        ),
                        (
                            "methods".to_owned(),
                            RecordProj::new(Variable::new("b"), "methods").into(),
                        ),
                    ])),
                    OpApp::new(object(), TypeVariable::new("M")),
                ),
            ),
        ),
    )
    .into()
}

pub fn send_get() -> Term {
    LambdaSub::new(
        "M",
        counter_m(),
        Lambda::new(
            "c",
            OpApp::new(object(), TypeVariable::new("M")),
            Unpack::new(
                "X",
                "b",
                Variable::new("c"),
                App::new(
                    RecordProj::new(RecordProj::new(Variable::new("b"), "methods"), "get"),
                    Variable::new("c"),
                ),
            ),
        ),
    )
    .into()
}

pub fn counter_r() -> Type {
    RecordTy::new(HashMap::<Label, Type>::from([(
        "x".to_owned(),
        Nat::new().into(),
    )]))
    .into()
}

pub fn counter_class() -> Term {
    Record::new(HashMap::<Var, Term>::from([
        (
            "get".to_owned(),
            Lambda::new("r", counter_r(), RecordProj::new(Variable::new("r"), "x")).into(),
        ),
        (
            "inc".to_owned(),
            Lambda::new(
                "r",
                counter_r(),
                Record::new(HashMap::<Var, Term>::from([(
                    "x".to_owned(),
                    Succ::new(RecordProj::new(Variable::new("r"), "x")).into(),
                )])),
            )
            .into(),
        ),
    ]))
    .into()
}

pub fn new_counter() -> Term {
    Pack::new(
        counter_r(),
        Record::new(HashMap::<Label, Term>::from([
            (
                "state".to_owned(),
                Record::new(HashMap::<Label, Term>::from([(
                    "x".to_owned(),
                    Num::new(0).into(),
                )]))
                .into(),
            ),
            ("methods".to_owned(), counter_class()),
        ])),
        counter(),
    )
    .into()
}

#[cfg(test)]
mod counter_tests {
    use super::{
        counter, counter_class, counter_m, counter_r, new_counter, object, send_get, send_inc,
    };
    use crate::types::Type;
    use common::{
        check::Typecheck,
        eval::Normalize,
        language::LanguageType,
        types::{ForallBounded, Fun, Nat, OpApp, TypeVariable},
    };

    #[test]
    fn check_send_inc() {
        let result = send_inc().check(&mut Default::default()).unwrap();
        let expected = ForallBounded::new(
            "M",
            counter_m(),
            Fun::new(
                OpApp::new(object(), TypeVariable::new("M")),
                OpApp::new(object(), TypeVariable::new("M")),
            ),
        )
        .into();
        result.check_equal(&expected).unwrap();
    }

    #[test]
    fn check_send_get() {
        let result = send_get().check(&mut Default::default()).unwrap();
        let ty: Type = ForallBounded::new(
            "M",
            counter_m(),
            Fun::new(OpApp::new(object(), TypeVariable::new("M")), Nat::new()),
        )
        .into();
        let expected = ty.normalize(&mut Default::default());
        result.check_equal(&expected).unwrap();
    }

    #[test]
    fn check_class() {
        let result = counter_class().check(&mut Default::default()).unwrap();
        let t: Type = OpApp::new(counter_m(), counter_r()).into();
        let expected = t.normalize(&mut Default::default());
        result.check_equal(&expected).unwrap();
    }

    #[test]
    fn check_new() {
        let result = new_counter()
            .check_start()
            .unwrap()
            .normalize(&mut Default::default());
        let expected = counter();
        result.check_equal(&expected).unwrap();
    }
}
