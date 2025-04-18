use super::{counter::counter_rep, reset_counter::reset_counter_class};
use crate::{terms::Term, types::Type};
use common::{
    terms::{App, Assign, Deref, Lambda, Let, Num, Pred, Record, RecordProj, Ref, Variable},
    types::{Fun, Nat, Record as RecordTy, Unit},
    Var,
};
use std::collections::HashMap;

pub fn ty_dec_counter() -> Type {
    RecordTy::new(HashMap::from([
        ("get".to_owned(), Fun::new(Unit, Nat)),
        ("inc".to_owned(), Fun::new(Unit, Unit)),
        ("reset".to_owned(), Fun::new(Unit, Unit)),
        ("dec".to_owned(), Fun::new(Nat, Unit)),
    ]))
    .into()
}

pub fn dec_counter_class() -> Term {
    Lambda::new(
        "r",
        counter_rep(),
        Let::new(
            "super",
            App::new(reset_counter_class(), Variable::new("r")),
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
                    RecordProj::new(Variable::new("super"), "reset").into(),
                ),
                (
                    "dec".to_owned(),
                    Lambda::new(
                        "n",
                        Nat,
                        Assign::new(
                            RecordProj::new(Variable::new("r"), "x"),
                            Pred::new(Deref::new(RecordProj::new(Variable::new("r"), "x"))),
                        ),
                    )
                    .into(),
                ),
            ])),
        ),
    )
    .into()
}

pub fn new_dec_counter() -> Term {
    Lambda::new(
        "_",
        Unit,
        Let::new(
            "r",
            Record::new(HashMap::<Var, Term>::from([(
                "x".to_owned(),
                Ref::new(Num::new(1)).into(),
            )])),
            App::new(dec_counter_class(), Variable::new("r")),
        ),
    )
    .into()
}

#[cfg(test)]
mod dec_counter_tests {
    use super::{counter_rep, dec_counter_class, new_dec_counter, ty_dec_counter};
    use crate::{
        objects::{counter::ty_counter, reset_counter::ty_reset_counter},
        types::Type,
        typing::is_subtype,
    };
    use common::Typecheck;

    #[test]
    fn subtype_reset() {
        assert!(is_subtype(&ty_dec_counter(), &ty_reset_counter()))
    }

    #[test]
    fn subtype_counter() {
        assert!(is_subtype(&ty_dec_counter(), &ty_counter()))
    }

    #[test]
    fn ty_dec_class() {
        let result = dec_counter_class().check(&mut Default::default()).unwrap();
        let expected = Fun::new(counter_rep(), ty_dec_counter());
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_new_dec() {
        let result = new_dec_counter().check(&mut Default::default()).unwrap();
        let expected = Fun::new(Unit, ty_dec_counter());
        assert_eq!(result, expected)
    }
}
