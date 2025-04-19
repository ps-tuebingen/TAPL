use crate::{terms::Term, types::Type};
use common::{
    terms::{App, Fix, Fold, Lambda, Let, Num, Pred, Record, RecordProj, Succ, Variable},
    types::{Fun, Mu, Nat, Record as RecordTy, TypeVariable, Unit as UnitTy},
    TypeVar, Var,
};
use std::collections::HashMap;

pub fn ty_counter() -> Type {
    Mu::new(
        "C",
        RecordTy::new(HashMap::<TypeVar, Type>::from([
            ("get".to_owned(), Nat.into()),
            (
                "inc".to_owned(),
                Fun::new(UnitTy, TypeVariable::new("C")).into(),
            ),
            (
                "dec".to_owned(),
                Fun::new(UnitTy, TypeVariable::new("C")).into(),
            ),
        ])),
    )
    .into()
}

pub fn new_counter() -> Term {
    Let::new(
        "create",
        Fix::new(Lambda::new(
            "f",
            Fun::new(
                RecordTy::new(HashMap::from([("x".to_owned(), Nat)])),
                ty_counter(),
            ),
            Lambda::new(
                "s",
                RecordTy::new(HashMap::<TypeVar, Type>::from([(
                    "x".to_owned(),
                    Nat.into(),
                )])),
                Fold::new(
                    Record::new(HashMap::<Var, Term>::from([
                        (
                            "get".to_owned(),
                            RecordProj::new(Variable::new("s"), "x").into(),
                        ),
                        (
                            "inc".to_owned(),
                            Lambda::new(
                                "_",
                                UnitTy,
                                App::new(
                                    Variable::new("f"),
                                    Record::new(HashMap::<Var, Term>::from([(
                                        "x".to_owned(),
                                        Succ::new(RecordProj::new(Variable::new("s"), "x")).into(),
                                    )])),
                                ),
                            )
                            .into(),
                        ),
                        (
                            "dec".to_owned(),
                            Lambda::new(
                                "_",
                                UnitTy,
                                App::new(
                                    Variable::new("f"),
                                    Record::new(HashMap::<Var, Term>::from([(
                                        "x".to_owned(),
                                        Pred::new(RecordProj::new(Variable::new("s"), "x")).into(),
                                    )])),
                                ),
                            )
                            .into(),
                        ),
                    ])),
                    ty_counter(),
                ),
            ),
        )),
        App::new(
            Variable::new("create"),
            Record::new(HashMap::<Var, Term>::from([(
                "x".to_owned(),
                Num::new(0).into(),
            )])),
        ),
    )
    .into()
}

#[cfg(test)]
mod counter_tests {
    use super::{new_counter, ty_counter};
    use common::Typecheck;

    #[test]
    fn check_new() {
        let result = new_counter().check(&mut Default::default()).unwrap();
        let expected = ty_counter();
        assert_eq!(result, expected)
    }
}
