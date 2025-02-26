use super::set_counter_open::set_counter_class;
use crate::{
    syntax::{App, Assign, Deref, Fix, Lambda, Let, Projection, Record, Ref, Succ, Term, Unit},
    types::Type,
};

pub fn ty_instr_counter() -> Type {
    Type::rec(vec![
        ("get", Type::fun(Type::Unit, Type::Nat)),
        ("set", Type::fun(Type::Nat, Type::Unit)),
        ("inc", Type::fun(Type::Unit, Type::Unit)),
        ("accesses", Type::fun(Type::Unit, Type::Nat)),
    ])
}

pub fn instr_counter_rep() -> Type {
    Type::rec(vec![
        ("x", Type::ref_ty(Type::Nat)),
        ("a", Type::ref_ty(Type::Nat)),
    ])
}

pub fn instr_counter_class() -> Term {
    Lambda::new(
        "r",
        instr_counter_rep(),
        Lambda::new(
            "self",
            Type::fun(Type::Unit, ty_instr_counter()),
            Lambda::new(
                "_",
                Type::Unit,
                Let::new(
                    "super",
                    App::new(
                        App::new(
                            App::new(set_counter_class(), "r".into()).into(),
                            "self".into(),
                        )
                        .into(),
                        Unit.into(),
                    )
                    .into(),
                    Record::new(vec![
                        ("get", Projection::new("super".into(), "get").into()),
                        (
                            "set",
                            Lambda::new(
                                "i",
                                Type::Nat,
                                Term::seq(
                                    Assign::new(
                                        Projection::new("r".into(), "a").into(),
                                        Succ::new(
                                            Deref::new(Projection::new("r".into(), "a").into())
                                                .into(),
                                        )
                                        .into(),
                                    )
                                    .into(),
                                    App::new(
                                        Projection::new("super".into(), "set").into(),
                                        "i".into(),
                                    )
                                    .into(),
                                ),
                            )
                            .into(),
                        ),
                        ("inc", Projection::new("super".into(), "inc").into()),
                        (
                            "accesses",
                            Lambda::new(
                                "_",
                                Type::Unit,
                                Deref::new(Projection::new("r".into(), "a").into()).into(),
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
        .into(),
    )
    .into()
}

pub fn new_instr_counter() -> Term {
    Lambda::new(
        "_",
        Type::Unit,
        Let::new(
            "r",
            Record::new(vec![
                ("x", Ref::new(1.into()).into()),
                ("a", Ref::new(0.into()).into()),
            ])
            .into(),
            App::new(
                Fix::new(App::new(instr_counter_class(), "r".into()).into()).into(),
                Unit.into(),
            )
            .into(),
        )
        .into(),
    )
    .into()
}

#[cfg(test)]
mod instr_counter_tests {
    use super::{instr_counter_class, instr_counter_rep, new_instr_counter, ty_instr_counter};
    use crate::{
        objects::set_counter::ty_set_counter,
        types::Type,
        typing::{is_subtype, Typecheck},
    };

    #[test]
    fn subtype_setcounter() {
        assert!(is_subtype(&ty_instr_counter(), &ty_set_counter()))
    }

    #[test]
    fn ty_class() {
        let result = instr_counter_class()
            .check(&mut Default::default())
            .unwrap();
        let expected = Type::fun(
            instr_counter_rep(),
            Type::fun(
                Type::fun(Type::Unit, ty_instr_counter()),
                Type::fun(Type::Unit, ty_instr_counter()),
            ),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_new() {
        let result = new_instr_counter().check(&mut Default::default()).unwrap();
        let expected = Type::fun(Type::Unit, ty_instr_counter());
        assert_eq!(result, expected)
    }
}
