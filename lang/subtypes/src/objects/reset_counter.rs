use super::counter::{counter_class, counter_rep};
use crate::{
    syntax::{App, Assign, Lambda, Let, Projection, Record, Ref, Term},
    types::Type,
};

pub fn ty_reset_counter() -> Type {
    Type::rec(vec![
        ("get", Type::fun(Type::Unit, Type::Nat)),
        ("inc", Type::fun(Type::Unit, Type::Unit)),
        ("reset", Type::fun(Type::Unit, Type::Unit)),
    ])
}

pub fn new_reset_counter() -> Term {
    Lambda::new(
        "_",
        Type::Unit,
        Let::new(
            "r",
            Record::new(vec![("x", Ref::new(1.into()).into())]).into(),
            App::new(reset_counter_class(), "r".into()).into(),
        )
        .into(),
    )
    .into()
}

pub fn reset_counter_class() -> Term {
    Lambda::new(
        "r",
        counter_rep(),
        Let::new(
            "super",
            App::new(counter_class(), "r".into()).into(),
            Record::new(vec![
                ("get", Projection::new("super".into(), "get").into()),
                ("inc", Projection::new("super".into(), "inc").into()),
                (
                    "reset",
                    Lambda::new(
                        "_",
                        Type::Unit,
                        Assign::new(Projection::new("r".into(), "x").into(), 1.into()).into(),
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
mod reset_counter_tests {
    use super::{counter_rep, new_reset_counter, reset_counter_class, ty_reset_counter};
    use crate::{
        objects::counter::ty_counter,
        types::Type,
        typing::{is_subtype, Typecheck},
    };

    #[test]
    fn subty_reset() {
        assert!(is_subtype(&ty_reset_counter(), &ty_counter()))
    }

    #[test]
    fn ty_new_reset_counter() {
        let result = new_reset_counter().check(&mut Default::default()).unwrap();
        let expected = Type::fun(Type::Unit, ty_reset_counter());
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_reset_class() {
        let result = reset_counter_class()
            .check(&mut Default::default())
            .unwrap();
        let expected = Type::fun(counter_rep(), ty_reset_counter());
        assert_eq!(result, expected)
    }
}
