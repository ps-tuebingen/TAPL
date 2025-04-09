use super::counter::counter_rep;
use crate::{
    syntax::{App, Assign, Deref, Fix, Lambda, Let, Projection, Record, Ref, Succ, Term, Unit},
    types::Type,
};

pub fn ty_set_counter() -> Type {
    Type::rec(vec![
        ("get", Type::fun(Type::Unit, Type::Nat)),
        ("set", Type::fun(Type::Nat, Type::Unit)),
        ("inc", Type::fun(Type::Unit, Type::Unit)),
    ])
}

pub fn set_counter_class() -> Term {
    Lambda::new(
        "r",
        counter_rep(),
        Fix::new(
            Lambda::new(
                "self",
                ty_set_counter(),
                Record::new(vec![
                    (
                        "get",
                        Lambda::new(
                            "_",
                            Type::Unit,
                            Deref::new(Projection::new("r".into(), "x").into()).into(),
                        )
                        .into(),
                    ),
                    (
                        "set",
                        Lambda::new(
                            "i",
                            Type::Nat,
                            Assign::new(Projection::new("r".into(), "x").into(), "i".into()).into(),
                        )
                        .into(),
                    ),
                    (
                        "inc",
                        Lambda::new(
                            "_",
                            Type::Unit,
                            App::new(
                                Projection::new("self".into(), "set").into(),
                                Succ::new(
                                    App::new(
                                        Projection::new("self".into(), "get").into(),
                                        Unit.into(),
                                    )
                                    .into(),
                                )
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
        .into(),
    )
    .into()
}

pub fn new_set_counter() -> Term {
    Lambda::new(
        "_",
        Type::Unit,
        Let::new(
            "r",
            Record::new(vec![("x", Ref::new(1.into()).into())]).into(),
            App::new(set_counter_class(), "r".into()).into(),
        )
        .into(),
    )
    .into()
}

#[cfg(test)]
mod set_counter_tests {
    use super::{new_set_counter, set_counter_class, ty_set_counter};
    use crate::{
        objects::counter::{counter_rep, ty_counter},
        types::Type,
        typing::is_subtype,
    };
    use common::Typecheck;

    #[test]
    fn subtype_counter() {
        assert!(is_subtype(&ty_set_counter(), &ty_counter()))
    }

    #[test]
    fn ty_set_class() {
        let result = set_counter_class().check(&mut Default::default()).unwrap();
        let expected = Type::fun(counter_rep(), ty_set_counter());
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_set_new() {
        let result = new_set_counter().check(&mut Default::default()).unwrap();
        let expected = Type::fun(Type::Unit, ty_set_counter());
        assert_eq!(result, expected)
    }
}
