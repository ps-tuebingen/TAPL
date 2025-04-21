use super::reset_counter::reset_counter_class;
use crate::{terms::Term, types::Type};
use common::{
    terms::{App, Assign, Deref, Lambda, Let, Record, RecordProj, Variable},
    types::{Fun, Nat, Record as RecordTy, Reference, Unit},
    Var,
};
use std::collections::HashMap;

pub fn ty_backup_counter() -> Type {
    RecordTy::new(HashMap::from([
        ("get".to_string(), Fun::new(Unit::new(), Nat::new())),
        ("inc".to_string(), Fun::new(Unit::new(), Unit::new())),
        ("reset".to_string(), Fun::new(Unit::new(), Unit::new())),
        ("backup".to_string(), Fun::new(Unit::new(), Unit::new())),
    ]))
    .into()
}

pub fn backup_counter_rep() -> Type {
    RecordTy::new(HashMap::from([
        ("x".to_owned(), Reference::new(Nat::new())),
        ("b".to_owned(), Reference::new(Nat::new())),
    ]))
    .into()
}

pub fn backup_counter_class() -> Term {
    Lambda::new(
        "r",
        backup_counter_rep(),
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
                    Lambda::new(
                        "_",
                        Unit::new(),
                        Assign::new(
                            RecordProj::new(Variable::new("r"), "x"),
                            Deref::new(RecordProj::new(Variable::new("r"), "b")),
                        ),
                    )
                    .into(),
                ),
                (
                    "backup".to_owned(),
                    Lambda::new(
                        "_",
                        Unit::new(),
                        Assign::new(
                            RecordProj::new(Variable::new("r"), "b"),
                            Deref::new(RecordProj::new(Variable::new("r"), "x")),
                        ),
                    )
                    .into(),
                ),
            ])),
        ),
    )
    .into()
}

#[cfg(test)]
mod backup_counter_tests {
    use super::{backup_counter_class, backup_counter_rep, ty_backup_counter};
    use crate::objects::{
        counter::{counter_rep, ty_counter},
        dec_counter::ty_dec_counter,
        reset_counter::ty_reset_counter,
    };
    use common::{
        check::{Subtypecheck, Typecheck},
        types::Fun,
    };

    #[test]
    fn subtype_counter() {
        ty_backup_counter()
            .check_subtype(&ty_counter(), &mut Default::default())
            .unwrap()
    }

    #[test]
    fn subtype_reset() {
        ty_backup_counter()
            .check_subtype(&ty_reset_counter(), &mut Default::default())
            .unwrap()
    }

    #[test]
    fn subtype_dec() {
        ty_backup_counter()
            .check_subtype(&ty_dec_counter(), &mut Default::default())
            .unwrap()
    }

    #[test]
    fn subtype_rep() {
        backup_counter_rep()
            .check_subtype(&counter_rep(), &mut Default::default())
            .unwrap()
    }

    #[test]
    fn ty_backup_class() {
        let result = backup_counter_class()
            .check(&mut Default::default())
            .unwrap();
        let expected = Fun::new(backup_counter_rep(), ty_backup_counter()).into();
        assert_eq!(result, expected)
    }
}
