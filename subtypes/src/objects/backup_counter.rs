use super::reset_counter::reset_counter_class;
use crate::{
    syntax::{App, Assign, Deref, Lambda, Let, Projection, Record, Term},
    types::Type,
};

pub fn ty_backup_counter() -> Type {
    Type::rec(vec![
        ("get", Type::fun(Type::Unit, Type::Nat)),
        ("inc", Type::fun(Type::Unit, Type::Unit)),
        ("reset", Type::fun(Type::Unit, Type::Unit)),
        ("backup", Type::fun(Type::Unit, Type::Unit)),
    ])
}

pub fn backup_counter_rep() -> Type {
    Type::rec(vec![
        ("x", Type::ref_ty(Type::Nat)),
        ("b", Type::ref_ty(Type::Nat)),
    ])
}

pub fn backup_counter_class() -> Term {
    Lambda::new(
        "r",
        backup_counter_rep(),
        Let::new(
            "super",
            App::new(reset_counter_class(), "r".into()).into(),
            Record::new(vec![
                ("get", Projection::new("super".into(), "get").into()),
                ("inc", Projection::new("super".into(), "inc").into()),
                (
                    "reset",
                    Lambda::new(
                        "_",
                        Type::Unit,
                        Assign::new(
                            Projection::new("r".into(), "x").into(),
                            Deref::new(Projection::new("r".into(), "b").into()).into(),
                        )
                        .into(),
                    )
                    .into(),
                ),
                (
                    "backup",
                    Lambda::new(
                        "_",
                        Type::Unit,
                        Assign::new(
                            Projection::new("r".into(), "b").into(),
                            Deref::new(Projection::new("r".into(), "x").into()).into(),
                        )
                        .into(),
                    )
                    .into(),
                ),
            ])
            .into(),
        )
        .into(),
    )
    .into()
}

#[cfg(test)]
mod backup_counter_tests {
    use super::{backup_counter_class, backup_counter_rep, ty_backup_counter};
    use crate::{
        objects::{
            counter::{counter_rep, ty_counter},
            dec_counter::ty_dec_counter,
            reset_counter::ty_reset_counter,
        },
        types::Type,
        typing::{is_subtype, Typecheck},
    };

    #[test]
    fn subtype_counter() {
        assert!(is_subtype(&ty_backup_counter(), &ty_counter()))
    }

    #[test]
    fn subtype_reset() {
        assert!(is_subtype(&ty_backup_counter(), &ty_reset_counter()))
    }

    #[test]
    fn subtype_dec() {
        assert!(!is_subtype(&ty_backup_counter(), &ty_dec_counter()))
    }

    #[test]
    fn subtype_rep() {
        assert!(is_subtype(&backup_counter_rep(), &counter_rep()))
    }

    #[test]
    fn ty_backup_class() {
        let result = backup_counter_class()
            .check(&mut Default::default())
            .unwrap();
        let expected = Type::fun(backup_counter_rep(), ty_backup_counter());
        assert_eq!(result, expected)
    }
}
