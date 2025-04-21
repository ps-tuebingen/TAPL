use super::counter::counter_rep;
use crate::{terms::Term, types::Type};
use common::{
    terms::{
        App, Assign, Deref, Fix, Lambda, Let, Num, Record, RecordProj, Ref, Succ, Unit, Variable,
    },
    types::{Fun, Nat, Record as RecordTy, Unit as UnitTy},
    Var,
};
use std::collections::HashMap;

pub fn ty_set_counter() -> Type {
    RecordTy::new(HashMap::from([
        ("get".to_owned(), Fun::new(UnitTy::new(), Nat::new())),
        ("set".to_owned(), Fun::new(Nat::new(), UnitTy::new())),
        ("inc".to_owned(), Fun::new(UnitTy::new(), UnitTy::new())),
    ]))
    .into()
}

pub fn set_counter_class() -> Term {
    Lambda::new(
        "r",
        counter_rep(),
        Fix::new(Lambda::new(
            "self",
            ty_set_counter(),
            Record::new(HashMap::<Var, Term>::from([
                (
                    "get".to_owned(),
                    Lambda::new(
                        "_",
                        UnitTy::new(),
                        Deref::new(RecordProj::new(Variable::new("r"), "x")),
                    )
                    .into(),
                ),
                (
                    "set".to_owned(),
                    Lambda::new(
                        "i",
                        Nat::new(),
                        Assign::new(RecordProj::new(Variable::new("r"), "x"), Variable::new("i")),
                    )
                    .into(),
                ),
                (
                    "inc".to_owned(),
                    Lambda::new(
                        "_",
                        UnitTy::new(),
                        App::new(
                            RecordProj::new(Variable::new("self"), "set"),
                            Succ::new(App::new(
                                RecordProj::new(Variable::new("self"), "get"),
                                Unit::new(),
                            )),
                        ),
                    )
                    .into(),
                ),
            ])),
        )),
    )
    .into()
}

pub fn new_set_counter() -> Term {
    Lambda::new(
        "_",
        UnitTy::new(),
        Let::new(
            "r",
            Record::new(HashMap::<Var, Term>::from([(
                "x".to_owned(),
                Ref::new(Num::new(1)).into(),
            )])),
            App::new(set_counter_class(), Variable::new("r")),
        ),
    )
    .into()
}

#[cfg(test)]
mod set_counter_tests {
    use super::{new_set_counter, set_counter_class, ty_set_counter};
    use crate::objects::counter::{counter_rep, ty_counter};
    use common::{
        check::{Subtypecheck, Typecheck},
        types::{Fun, Unit as UnitTy},
    };

    #[test]
    fn subtype_counter() {
        ty_set_counter()
            .check_subtype(&ty_counter(), &mut Default::default())
            .unwrap()
    }

    #[test]
    fn ty_set_class() {
        let result = set_counter_class().check(&mut Default::default()).unwrap();
        let expected = Fun::new(counter_rep(), ty_set_counter()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_set_new() {
        let result = new_set_counter().check(&mut Default::default()).unwrap();
        let expected = Fun::new(UnitTy::new(), ty_set_counter()).into();
        assert_eq!(result, expected)
    }
}
