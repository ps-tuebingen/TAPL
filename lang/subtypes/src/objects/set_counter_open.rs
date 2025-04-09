use super::{counter::counter_rep, set_counter::ty_set_counter};
use crate::{
    syntax::{App, Assign, Deref, Fix, Lambda, Let, Projection, Record, Ref, Succ, Term, Unit},
    types::Type,
};

pub fn set_counter_class() -> Term {
    Lambda::new(
        "r",
        counter_rep(),
        Lambda::new(
            "self",
            Type::fun(Type::Unit, ty_set_counter()),
            Lambda::new(
                "_",
                Type::Unit,
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
                                Projection::new(App::new("self".into(), Unit.into()).into(), "set")
                                    .into(),
                                Succ::new(
                                    App::new(
                                        Projection::new(
                                            App::new("self".into(), Unit.into()).into(),
                                            "get",
                                        )
                                        .into(),
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
            App::new(
                Fix::new(App::new(set_counter_class(), "r".into()).into()).into(),
                Unit.into(),
            )
            .into(),
        )
        .into(),
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
        let expected = Type::fun(
            counter_rep(),
            Type::fun(
                Type::fun(Type::Unit, ty_set_counter()),
                Type::fun(Type::Unit, ty_set_counter()),
            ),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_set_new() {
        let result = new_set_counter().check(&mut Default::default()).unwrap();
        let expected = Type::fun(Type::Unit, ty_set_counter());
        assert_eq!(result, expected)
    }
}
