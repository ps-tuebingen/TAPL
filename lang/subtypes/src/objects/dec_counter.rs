use super::{counter::counter_rep, reset_counter::reset_counter_class};
use crate::{
    syntax::{App, Assign, Deref, Lambda, Let, Pred, Projection, Record, Ref, Term},
    types::Type,
};

pub fn ty_dec_counter() -> Type {
    Type::rec(vec![
        ("get", Type::fun(Type::Unit, Type::Nat)),
        ("inc", Type::fun(Type::Unit, Type::Unit)),
        ("reset", Type::fun(Type::Unit, Type::Unit)),
        ("dec", Type::fun(Type::Nat, Type::Unit)),
    ])
}

pub fn dec_counter_class() -> Term {
    Lambda::new(
        "r",
        counter_rep(),
        Let::new(
            "super",
            App::new(reset_counter_class(), "r".into()).into(),
            Record::new(vec![
                ("get", Projection::new("super".into(), "get").into()),
                ("inc", Projection::new("super".into(), "inc").into()),
                ("reset", Projection::new("super".into(), "reset").into()),
                (
                    "dec",
                    Lambda::new(
                        "n",
                        Type::Nat,
                        Assign::new(
                            Projection::new("r".into(), "x").into(),
                            Pred::new(Deref::new(Projection::new("r".into(), "x").into()).into())
                                .into(),
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

pub fn new_dec_counter() -> Term {
    Lambda::new(
        "_",
        Type::Unit,
        Let::new(
            "r",
            Record::new(vec![("x", Ref::new(1.into()).into())]).into(),
            App::new(dec_counter_class(), "r".into()).into(),
        )
        .into(),
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
        let expected = Type::fun(counter_rep(), ty_dec_counter());
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_new_dec() {
        let result = new_dec_counter().check(&mut Default::default()).unwrap();
        let expected = Type::fun(Type::Unit, ty_dec_counter());
        assert_eq!(result, expected)
    }
}
