use crate::{
    syntax::{App, Lambda, Term, TyApp, TyLambda},
    types::Type,
};

pub fn c_nat() -> Type {
    Type::forall(
        "X",
        Type::fun(
            Type::fun("X".into(), "X".into()),
            Type::fun("X".into(), "X".into()),
        ),
    )
}

pub fn c0() -> Term {
    TyLambda::new(
        "X",
        Lambda::new(
            "s",
            Type::fun("X".into(), "X".into()),
            Lambda::new("z", "X".into(), "z".into()).into(),
        )
        .into(),
    )
    .into()
}

pub fn c1() -> Term {
    TyLambda::new(
        "X",
        Lambda::new(
            "s",
            Type::fun("X".into(), "X".into()),
            Lambda::new("z", "X".into(), App::new("s".into(), "z".into()).into()).into(),
        )
        .into(),
    )
    .into()
}

pub fn c2() -> Term {
    TyLambda::new(
        "X",
        Lambda::new(
            "s",
            Type::fun("X".into(), "X".into()),
            Lambda::new(
                "z",
                "X".into(),
                App::new("s".into(), App::new("s".into(), "z".into()).into()).into(),
            )
            .into(),
        )
        .into(),
    )
    .into()
}

pub fn csucc() -> Term {
    Lambda::new(
        "n",
        c_nat(),
        TyLambda::new(
            "X",
            Lambda::new(
                "s",
                Type::fun("X".into(), "X".into()),
                Lambda::new(
                    "z",
                    "X".into(),
                    App::new(
                        "s".into(),
                        App::new(
                            App::new(TyApp::new("n".into(), "X".into()).into(), "s".into()).into(),
                            "z".into(),
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

pub fn cplus() -> Term {
    Lambda::new(
        "m",
        c_nat(),
        Lambda::new(
            "n",
            c_nat(),
            App::new(
                App::new(TyApp::new("m".into(), c_nat()).into(), csucc()).into(),
                "n".into(),
            )
            .into(),
        )
        .into(),
    )
    .into()
}

pub fn ctimes() -> Term {
    Lambda::new(
        "m",
        c_nat(),
        Lambda::new(
            "n",
            c_nat(),
            TyLambda::new(
                "X",
                Lambda::new(
                    "s",
                    Type::fun("X".into(), "X".into()),
                    App::new(
                        TyApp::new("n".into(), "X".into()).into(),
                        App::new(TyApp::new("m".into(), "X".into()).into(), "s".into()).into(),
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

pub fn cexp() -> Term {
    Lambda::new(
        "m",
        c_nat(),
        Lambda::new(
            "n",
            c_nat(),
            TyLambda::new(
                "X",
                App::new(
                    TyApp::new("n".into(), Type::fun("X".into(), "X".into())).into(),
                    TyApp::new("m".into(), "X".into()).into(),
                )
                .into(),
            )
            .into(),
        )
        .into(),
    )
    .into()
}

#[cfg(test)]
mod nat_tests {
    use super::{c0, c1, c2, c_nat, cexp, cplus, csucc, ctimes};
    use crate::{check::Check, types::Type};

    #[test]
    fn ty_c0() {
        let result = c0().check(&mut Default::default()).unwrap();
        let expected = c_nat();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_c1() {
        let result = c1().check(&mut Default::default()).unwrap();
        let expected = c_nat();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_c2() {
        let result = c2().check(&mut Default::default()).unwrap();
        let expected = c_nat();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_succ() {
        let result = csucc().check(&mut Default::default()).unwrap();
        let expected = Type::fun(c_nat(), c_nat());
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_cplus() {
        let result = cplus().check(&mut Default::default()).unwrap();
        let expected = Type::fun(c_nat(), Type::fun(c_nat(), c_nat()));
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_ctimes() {
        let result = ctimes().check(&mut Default::default()).unwrap();
        let expected = Type::fun(c_nat(), Type::fun(c_nat(), c_nat()));
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_cexp() {
        let result = cexp().check(&mut Default::default()).unwrap();
        let expected = Type::fun(c_nat(), Type::fun(c_nat(), c_nat()));
        assert_eq!(result, expected)
    }
}
