use super::{
    counter::{counter_class, counter_r},
    object,
};
use crate::{terms::Term, types::Type};
use common::{
    kinds::Kind,
    terms::{App, Lambda, LambdaSub, Let, Num, Pack, Record, RecordProj, Unpack, Variable},
    types::{Fun, Nat, OpApp, OpLambda, Record as RecordTy, TypeVariable},
    Label, Var,
};
use std::collections::HashMap;

pub fn reset_counter_m() -> Type {
    OpLambda::new(
        "R",
        Kind::Star,
        RecordTy::new(HashMap::<Label, Type>::from([
            (
                "get".to_owned(),
                Fun::new(TypeVariable::new("R"), Nat::new()).into(),
            ),
            (
                "inc".to_owned(),
                Fun::new(TypeVariable::new("R"), TypeVariable::new("R")).into(),
            ),
            (
                "reset".to_owned(),
                Fun::new(TypeVariable::new("R"), TypeVariable::new("R")).into(),
            ),
        ])),
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
        Record::new(HashMap::<Var, Term>::from([
            (
                "get".to_owned(),
                RecordProj::new(Variable::new("super"), "get").into(),
            ),
            (
                "inc".to_owned(),
                RecordProj::new(Variable::new("super"), "inc").into(),
            ),
            (
                "reset".to_owned(),
                Lambda::new(
                    "r",
                    counter_r(),
                    Record::new(HashMap::<Var, Term>::from([(
                        "x".to_owned(),
                        Num::new(0).into(),
                    )])),
                )
                .into(),
            ),
        ])),
    )
    .into()
}

pub fn send_reset() -> Term {
    LambdaSub::new(
        "M",
        reset_counter_m(),
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
                                    "reset",
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

#[cfg(test)]
mod reset_counter_tests {
    use super::{object, reset_counter, reset_counter_class, reset_counter_m, send_reset};
    use crate::{
        objects::counter::{counter, counter_m, counter_r},
        types::Type,
    };
    use common::{
        check::{Subtypecheck, Typecheck},
        eval::Normalize,
        language::LanguageType,
        types::{ForallBounded, Fun, OpApp, TypeVariable},
    };

    #[test]
    fn reset_counter_sub() {
        reset_counter()
            .check_subtype(&counter(), &mut Default::default())
            .unwrap();
    }

    #[test]
    fn reset_counter_m_sub() {
        reset_counter_m()
            .check_subtype(&counter_m(), &mut Default::default())
            .unwrap();
    }

    #[test]
    fn check_class() {
        let result = reset_counter_class()
            .check(&mut Default::default())
            .unwrap();
        let ty: Type = OpApp::new(reset_counter_m(), counter_r()).into();
        let expected = ty.normalize();
        result.check_equal(&expected).unwrap();
    }
    #[test]
    fn check_send_reset() {
        let result = send_reset().check(&mut Default::default()).unwrap();
        let expected = ForallBounded::new(
            "M",
            reset_counter_m(),
            Fun::new(
                OpApp::new(object(), TypeVariable::new("M")),
                OpApp::new(object(), TypeVariable::new("M")),
            ),
        )
        .into();
        result.check_equal(&expected).unwrap();
    }
}
