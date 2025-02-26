use crate::{
    syntax::{App, Lambda, Term, TyApp, TyLambda},
    types::Type,
};

pub fn c_bool() -> Type {
    Type::forall(
        "X",
        Type::fun("X".into(), Type::fun("X".into(), "X".into())),
    )
}

pub fn tru() -> Term {
    TyLambda::new(
        "X",
        Lambda::new(
            "t",
            "X".into(),
            Lambda::new("f", "X".into(), "t".into()).into(),
        )
        .into(),
    )
    .into()
}

pub fn fls() -> Term {
    TyLambda::new(
        "X",
        Lambda::new(
            "x",
            "X".into(),
            Lambda::new("f", "X".into(), "f".into()).into(),
        )
        .into(),
    )
    .into()
}

pub fn not() -> Term {
    Lambda::new(
        "b",
        c_bool(),
        TyLambda::new(
            "X",
            Lambda::new(
                "t",
                "X".into(),
                Lambda::new(
                    "f",
                    "X".into(),
                    App::new(
                        App::new(TyApp::new("b".into(), "X".into()).into(), "f".into()).into(),
                        "t".into(),
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

#[cfg(test)]
mod bool_tests {
    use super::{c_bool, fls, not, tru};
    use crate::{check::Check, types::Type};

    #[test]
    fn ty_tru() {
        let result = tru().check(&mut Default::default()).unwrap();
        let expected = c_bool();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_fls() {
        let result = fls().check(&mut Default::default()).unwrap();
        let expected = c_bool();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_not() {
        let result = not().check(&mut Default::default()).unwrap();
        let expected = Type::fun(c_bool(), c_bool());
        assert_eq!(result, expected)
    }
}
