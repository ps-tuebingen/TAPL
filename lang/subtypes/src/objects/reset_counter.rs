use super::counter::{counter_class, counter_rep};
use crate::{terms::Term, types::Type};
use common::{
    terms::{App, Assign, Lambda, Let, Num, Record, RecordProj, Ref, Variable},
    types::{Fun, Nat, Record as RecordTy, Unit},
    Var,
};
use std::collections::HashMap;

pub fn ty_reset_counter() -> Type {
    RecordTy::new(HashMap::from([
        ("get".to_owned(), Fun::new(Unit, Nat)),
        ("inc".to_owned(), Fun::new(Unit, Unit)),
        ("reset".to_owned(), Fun::new(Unit, Unit)),
    ]))
    .into()
}

pub fn new_reset_counter() -> Term {
    Lambda::new(
        "_",
        Unit,
        Let::new(
            "r",
            Record::new(HashMap::<Var, Term>::from([(
                "x".to_owned(),
                Ref::new(Num::new(1)).into(),
            )])),
            App::new(reset_counter_class(), Variable::new("r")),
        ),
    )
    .into()
}

pub fn reset_counter_class() -> Term {
    Lambda::new(
        "r",
        counter_rep(),
        Let::new(
            "super",
            App::new(counter_class(), Variable::new("r")),
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
                        "_",
                        Unit,
                        Assign::new(RecordProj::new(Variable::new("r"), "x"), Num::new(1)),
                    )
                    .into(),
                ),
            ])),
        ),
    )
    .into()
}

#[cfg(test)]
mod reset_counter_tests {
    use super::{counter_rep, new_reset_counter, reset_counter_class, ty_reset_counter};
    use crate::{objects::counter::ty_counter, types::Type, typing::is_subtype};
    use common::Typecheck;

    #[test]
    fn subty_reset() {
        assert!(is_subtype(&ty_reset_counter(), &ty_counter()))
    }

    #[test]
    fn ty_new_reset_counter() {
        let result = new_reset_counter().check(&mut Default::default()).unwrap();
        let expected = Fun::new(Unit, ty_reset_counter());
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_reset_class() {
        let result = reset_counter_class()
            .check(&mut Default::default())
            .unwrap();
        let expected = Fun::new(counter_rep(), ty_reset_counter());
        assert_eq!(result, expected)
    }
}
