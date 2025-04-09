use crate::{
    syntax::{App, Lambda, Term, TyApp, TyLambda},
    types::Type,
};

pub mod bool;
pub mod list;
pub mod nat;

pub fn id() -> Term {
    TyLambda::new("X", Lambda::new("x", "X".into(), "x".into()).into()).into()
}

pub fn double() -> Term {
    TyLambda::new(
        "X",
        Lambda::new(
            "f",
            Type::fun("X".into(), "X".into()),
            Lambda::new(
                "a",
                "X".into(),
                App::new("f".into(), App::new("f".into(), "a".into()).into()).into(),
            )
            .into(),
        )
        .into(),
    )
    .into()
}

pub fn self_app() -> Term {
    Lambda::new(
        "x",
        Type::forall("X", Type::fun("X".into(), "X".into())),
        App::new(
            TyApp::new(
                "x".into(),
                Type::forall("X", Type::fun("X".into(), "X".into())),
            )
            .into(),
            "x".into(),
        )
        .into(),
    )
    .into()
}

pub fn quadruple() -> Term {
    TyLambda::new(
        "X",
        App::new(
            TyApp::new(double(), Type::fun("X".into(), "X".into())).into(),
            TyApp::new(double(), "X".into()).into(),
        )
        .into(),
    )
    .into()
}

#[cfg(test)]
mod examples_tests {
    use super::{double, id, quadruple, self_app};
    use crate::types::Type;
    use common::Typecheck;

    #[test]
    fn ty_id() {
        let result = id().check(&mut Default::default()).unwrap();
        let expected = Type::forall("X", Type::fun("X".into(), "X".into()));
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_dobule() {
        let result = double().check(&mut Default::default()).unwrap();
        let expected = Type::forall(
            "X",
            Type::fun(
                Type::fun("X".into(), "X".into()),
                Type::fun("X".into(), "X".into()),
            ),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_selfapp() {
        let result = self_app().check(&mut Default::default()).unwrap();
        let expected = Type::fun(
            Type::forall("X", Type::fun("X".into(), "X".into())),
            Type::forall("X", Type::fun("X".into(), "X".into())),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_quadruple() {
        let result = quadruple().check(&mut Default::default()).unwrap();
        let expected = Type::forall(
            "X",
            Type::fun(
                Type::fun("X".into(), "X".into()),
                Type::fun("X".into(), "X".into()),
            ),
        );
        assert_eq!(result, expected)
    }
}
