use super::bool::{c_bool, fls, tru};
use crate::{
    syntax::{App, Lambda, Term, TyApp, TyLambda},
    types::Type,
};

pub fn list(x: Type) -> Type {
    Type::forall(
        "R",
        Type::fun(
            Type::fun(x, Type::fun("R".into(), "R".into())),
            Type::fun("R".into(), "R".into()),
        ),
    )
}

pub fn nil() -> Term {
    TyLambda::new(
        "X",
        TyLambda::new(
            "R",
            Lambda::new(
                "c",
                Type::fun("X".into(), Type::fun("R".into(), "R".into())),
                Lambda::new("n", "R".into(), "n".into()).into(),
            )
            .into(),
        )
        .into(),
    )
    .into()
}

pub fn cons() -> Term {
    TyLambda::new(
        "X",
        Lambda::new(
            "hd",
            "X".into(),
            Lambda::new(
                "tl",
                list("X".into()),
                TyLambda::new(
                    "R",
                    Lambda::new(
                        "c",
                        Type::fun("X".into(), Type::fun("R".into(), "R".into())),
                        Lambda::new(
                            "n",
                            "R".into(),
                            App::new(
                                App::new("c".into(), "hd".into()).into(),
                                App::new(
                                    App::new(
                                        TyApp::new("tl".into(), "R".into()).into(),
                                        "c".into(),
                                    )
                                    .into(),
                                    "n".into(),
                                )
                                .into(),
                            )
                            .into(),
                        )
                        .into(),
                    )
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

pub fn isnil() -> Term {
    TyLambda::new(
        "X",
        Lambda::new(
            "l",
            list("X".into()),
            App::new(
                App::new(
                    TyApp::new("l".into(), c_bool()).into(),
                    Lambda::new("hd", "X".into(), Lambda::new("tl", c_bool(), fls()).into()).into(),
                )
                .into(),
                tru(),
            )
            .into(),
        )
        .into(),
    )
    .into()
}

#[cfg(test)]
mod list_tests {
    use super::{c_bool, cons, isnil, list, nil};
    use crate::{check::Check, types::Type};

    #[test]
    fn ty_nil() {
        let result = nil().check(&mut Default::default()).unwrap();
        let expected = Type::forall("X", list("X".into()));
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_cons() {
        let result = cons().check(&mut Default::default()).unwrap();
        let expected = Type::forall(
            "X",
            Type::fun("X".into(), Type::fun(list("X".into()), list("X".into()))),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_isnil() {
        let result = isnil().check(&mut Default::default()).unwrap();
        let expected = Type::forall("X", Type::fun(list("X".into()), c_bool()));
        assert_eq!(result, expected)
    }
}
