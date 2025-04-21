use crate::{terms::Term, types::Type};
use common::{
    terms::{App, Assign, Deref, Lambda, Let, Num, Record, RecordProj, Ref, Succ, Variable},
    types::{Fun, Nat, Record as RecordTy, Reference, Unit},
    Var,
};
use std::collections::HashMap;

pub fn ty_counter() -> Type {
    RecordTy::new(HashMap::from([
        ("get".to_owned(), Fun::new(Unit::new(), Nat::new())),
        ("inc".to_owned(), Fun::new(Unit::new(), Unit::new())),
    ]))
    .into()
}

pub fn counter_rep() -> Type {
    RecordTy::new(HashMap::from([(
        "x".to_owned(),
        Reference::new(Nat::new()),
    )]))
    .into()
}

pub fn counter() -> Term {
    Let::new(
        "r",
        Record::new(HashMap::<Var, Term>::from([(
            "x".to_owned(),
            Ref::new(Num::new(1)).into(),
        )])),
        Record::new(HashMap::<Var, Term>::from([
            (
                "get".to_owned(),
                Lambda::new(
                    "_",
                    Unit::new(),
                    Deref::new(RecordProj::new(Variable::new("r"), "x")),
                )
                .into(),
            ),
            (
                "inc".to_owned(),
                Lambda::new(
                    "_",
                    Unit::new(),
                    Assign::new(
                        RecordProj::new(Variable::new("r"), "x"),
                        Succ::new(Deref::new(RecordProj::new(Variable::new("r"), "x"))),
                    ),
                )
                .into(),
            ),
        ])),
    )
    .into()
}

pub fn new_counter() -> Term {
    Lambda::new(
        "_",
        Unit::new(),
        Let::new(
            "r",
            Record::new(HashMap::<Var, Term>::from([(
                "x".to_owned(),
                Ref::new(Num::new(1)).into(),
            )])),
            App::new(counter_class(), Variable::new("r")),
        ),
    )
    .into()
}

pub fn counter_class() -> Term {
    Lambda::new(
        "r",
        counter_rep(),
        Record::new(HashMap::<Var, Term>::from([
            (
                "get".to_owned(),
                Lambda::new(
                    "_",
                    Unit::new(),
                    Deref::new(RecordProj::new(Variable::new("r"), "x")),
                )
                .into(),
            ),
            (
                "inc".to_owned(),
                Lambda::new(
                    "_",
                    Unit::new(),
                    Assign::new(
                        RecordProj::new(Variable::new("r"), "x"),
                        Succ::new(Deref::new(RecordProj::new(Variable::new("r"), "x"))),
                    ),
                )
                .into(),
            ),
        ])),
    )
    .into()
}

#[cfg(test)]
mod counter_tests {
    use super::{counter, counter_class, counter_rep, new_counter, ty_counter};
    use crate::types::Type;
    use common::Typecheck;

    #[test]
    fn check_counter() {
        let result = counter().check(&mut Default::default()).unwrap();
        let expected = ty_counter();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_new_counter() {
        let result = new_counter().check(&mut Default::default()).unwrap();
        let expected = Type::fun(Type::Unit::new(), ty_counter());
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_class() {
        let result = counter_class().check(&mut Default::default()).unwrap();
        let expected = Type::fun(counter_rep(), ty_counter());
        assert_eq!(result, expected)
    }
}
