use crate::{
    syntax::{App, Assign, Deref, Lambda, Let, Projection, Record, Ref, Succ, Term},
    types::Type,
};

pub fn ty_counter() -> Type {
    Type::rec(vec![
        ("get", Type::fun(Type::Unit, Type::Nat)),
        ("inc", Type::fun(Type::Unit, Type::Unit)),
    ])
}

pub fn counter_rep() -> Type {
    Type::rec(vec![("x", Type::ref_ty(Type::Nat))])
}

pub fn counter() -> Term {
    Let::new(
        "r",
        Record::new(vec![("x", Ref::new(1.into()).into())]).into(),
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
                "inc",
                Lambda::new(
                    "_",
                    Type::Unit,
                    Assign::new(
                        Projection::new("r".into(), "x").into(),
                        Succ::new(Deref::new(Projection::new("r".into(), "x").into()).into())
                            .into(),
                    )
                    .into(),
                )
                .into(),
            ),
        ])
        .into(),
    )
    .into()
}

pub fn new_counter() -> Term {
    Lambda::new(
        "_",
        Type::Unit,
        Let::new(
            "r",
            Record::new(vec![("x", Ref::new(1.into()).into())]).into(),
            App::new(counter_class(), "r".into()).into(),
        )
        .into(),
    )
    .into()
}

pub fn counter_class() -> Term {
    Lambda::new(
        "r",
        counter_rep(),
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
                "inc",
                Lambda::new(
                    "_",
                    Type::Unit,
                    Assign::new(
                        Projection::new("r".into(), "x").into(),
                        Succ::new(Deref::new(Projection::new("r".into(), "x").into()).into())
                            .into(),
                    )
                    .into(),
                )
                .into(),
            ),
        ])
        .into(),
    )
    .into()
}

#[cfg(test)]
mod counter_tests {
    use super::{counter, counter_class, counter_rep, new_counter, ty_counter};
    use crate::{types::Type, typing::Typecheck};

    #[test]
    fn check_counter() {
        let result = counter().check(&mut Default::default()).unwrap();
        let expected = ty_counter();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_new_counter() {
        let result = new_counter().check(&mut Default::default()).unwrap();
        let expected = Type::fun(Type::Unit, ty_counter());
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_class() {
        let result = counter_class().check(&mut Default::default()).unwrap();
        let expected = Type::fun(counter_rep(), ty_counter());
        assert_eq!(result, expected)
    }
}
