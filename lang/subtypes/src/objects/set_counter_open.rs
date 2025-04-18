use super::{counter::counter_rep, set_counter::ty_set_counter};
use crate::terms::Term;
use common::{
    terms::{
        App, Assign, Deref, Fix, Lambda, Let, Num, Record, RecordProj, Ref, Succ, Unit, Variable,
    },
    types::{Fun, Nat, Unit as UnitTy},
    Var,
};
use std::collections::HashMap;

pub fn set_counter_class() -> Term {
    Lambda::new(
        "r",
        counter_rep(),
        Lambda::new(
            "self",
            Fun::new(UnitTy, ty_set_counter()),
            Lambda::new(
                "_",
                UnitTy,
                Record::new(HashMap::<Var, Term>::from([
                    (
                        "get".to_owned(),
                        Lambda::new(
                            "_",
                            UnitTy,
                            Deref::new(RecordProj::new(Variable::new("r"), "x")),
                        )
                        .into(),
                    ),
                    (
                        "set".to_owned(),
                        Lambda::new(
                            "i",
                            Nat,
                            Assign::new(
                                RecordProj::new(Variable::new("r"), "x"),
                                Variable::new("i"),
                            ),
                        )
                        .into(),
                    ),
                    (
                        "inc".to_owned(),
                        Lambda::new(
                            "_",
                            UnitTy,
                            App::new(
                                RecordProj::new(
                                    App::new(Variable::new("self"), Unit::new()),
                                    "set",
                                ),
                                Succ::new(App::new(
                                    RecordProj::new(
                                        App::new(Variable::new("self"), Unit::new()),
                                        "get",
                                    ),
                                    Unit::new(),
                                )),
                            ),
                        )
                        .into(),
                    ),
                ])),
            ),
        ),
    )
    .into()
}

pub fn new_set_counter() -> Term {
    Lambda::new(
        "_",
        UnitTy,
        Let::new(
            "r",
            Record::new(HashMap::<Var, Term>::from([(
                "x".to_owned(),
                Ref::new(Num::new(1)).into(),
            )])),
            App::new(
                Fix::new(App::new(set_counter_class(), Variable::new("r"))),
                Unit::new(),
            ),
        ),
    )
    .into()
}

#[cfg(test)]
mod set_counter_open_tests {
    use super::{new_set_counter, set_counter_class, ty_set_counter};
    use crate::{objects::counter::counter_rep, types::Type};
    use common::Typecheck;

    #[test]
    fn ty_set_class() {
        let result = set_counter_class().check(&mut Default::default()).unwrap();
        let expected = Fun::new(
            counter_rep(),
            Fun::new(
                Fun::new(Unit, ty_set_counter()),
                Fun::new(Unit, ty_set_counter()),
            ),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_set_new() {
        let result = new_set_counter().check(&mut Default::default()).unwrap();
        let expected = Fun::new(Unit, ty_set_counter());
        assert_eq!(result, expected)
    }
}
