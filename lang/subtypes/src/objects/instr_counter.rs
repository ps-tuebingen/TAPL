use super::set_counter_open::set_counter_class;
use crate::{terms::Term, types::Type};
use common::{
    terms::{
        App, Assign, Deref, Fix, Lambda, Let, Num, Record, RecordProj, Ref, Succ, Unit, Variable,
    },
    types::{Fun, Nat, Record as RecordTy, Reference, Unit as UnitTy},
    Var,
};
use std::collections::HashMap;

pub fn ty_instr_counter() -> Type {
    RecordTy::new(HashMap::from([
        ("get".to_owned(), Fun::new(UnitTy, Nat)),
        ("set".to_owned(), Fun::new(Nat, UnitTy)),
        ("inc".to_owned(), Fun::new(UnitTy, UnitTy)),
        ("accesses".to_owned(), Fun::new(UnitTy, Nat)),
    ]))
    .into()
}

pub fn instr_counter_rep() -> Type {
    RecordTy::new(HashMap::from([
        ("x".to_owned(), Reference::new(Nat)),
        ("a".to_owned(), Reference::new(Nat)),
    ]))
    .into()
}

pub fn instr_counter_class() -> Term {
    Lambda::new(
        "r",
        instr_counter_rep(),
        Lambda::new(
            "self",
            Fun::new(UnitTy, ty_instr_counter()),
            Lambda::new(
                "_",
                UnitTy,
                Let::new(
                    "super",
                    App::new(
                        App::new(
                            App::new(set_counter_class(), Variable::new("r")),
                            Variable::new("self"),
                        ),
                        Unit::new(),
                    ),
                    Record::new(HashMap::<Var, Term>::from([
                        (
                            "get".to_owned(),
                            RecordProj::new(Variable::new("super"), "get").into(),
                        ),
                        (
                            "set".to_owned(),
                            Lambda::new(
                                "i",
                                Nat,
                                App::seq(
                                    Assign::new(
                                        RecordProj::new(Variable::new("r"), "a"),
                                        Succ::new(Deref::new(RecordProj::new(
                                            Variable::new("r"),
                                            "a",
                                        ))),
                                    ),
                                    App::new(
                                        RecordProj::new(Variable::new("super"), "set"),
                                        Variable::new("i"),
                                    ),
                                ),
                            )
                            .into(),
                        ),
                        (
                            "inc".to_owned(),
                            RecordProj::new(Variable::new("super"), "inc").into(),
                        ),
                        (
                            "accesses".to_owned(),
                            Lambda::new(
                                "_",
                                UnitTy,
                                Deref::new(RecordProj::new(Variable::new("r"), "a")),
                            )
                            .into(),
                        ),
                    ])),
                ),
            ),
        ),
    )
    .into()
}

pub fn new_instr_counter() -> Term {
    Lambda::new(
        "_",
        UnitTy,
        Let::new(
            "r",
            Record::new(HashMap::<Var, Term>::from([
                ("x".to_owned(), Ref::new(Num::new(1)).into()),
                ("a".to_owned(), Ref::new(Num::new(0)).into()),
            ])),
            App::new(
                Fix::new(App::new(instr_counter_class(), Variable::new("r"))),
                Unit::new(),
            ),
        ),
    )
    .into()
}

#[cfg(test)]
mod instr_counter_tests {
    use super::{instr_counter_class, instr_counter_rep, new_instr_counter, ty_instr_counter};
    use crate::{objects::set_counter::ty_set_counter, types::Type, typing::is_subtype};
    use common::Typecheck;

    #[test]
    fn subtype_setcounter() {
        assert!(is_subtype(&ty_instr_counter(), &ty_set_counter()))
    }

    #[test]
    fn ty_class() {
        let result = instr_counter_class()
            .check(&mut Default::default())
            .unwrap();
        let expected = Fun::new(
            instr_counter_rep(),
            Fun::new(
                Fun::new(UnitTy, ty_instr_counter()),
                Fun::new(UnitTy, ty_instr_counter()),
            ),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_new() {
        let result = new_instr_counter().check(&mut Default::default()).unwrap();
        let expected = Fun::new(UnitTy, ty_instr_counter());
        assert_eq!(result, expected)
    }
}
