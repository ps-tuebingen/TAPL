use crate::terms::Term;
use common::{
    kinds::Kind,
    terms::{App, Lambda, TyApp, TyLambda, Variable},
    types::{Forall, Fun, TypeVariable},
};

pub mod bool;
pub mod list;
pub mod nat;

pub fn id() -> Term {
    TyLambda::new(
        "X",
        Kind::Star,
        Lambda::new("x", TypeVariable::new("X"), Variable::new("x")),
    )
    .into()
}

pub fn double() -> Term {
    TyLambda::new(
        "X",
        Kind::Star,
        Lambda::new(
            "f",
            Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
            Lambda::new(
                "a",
                TypeVariable::new("X"),
                App::new(
                    Variable::new("f"),
                    App::new(Variable::new("f"), Variable::new("a")),
                ),
            ),
        ),
    )
    .into()
}

pub fn self_app() -> Term {
    Lambda::new(
        "x",
        Forall::new(
            "X",
            Kind::Star,
            Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
        ),
        App::new(
            TyApp::new(
                Variable::new("x"),
                Forall::new(
                    "X",
                    Kind::Star,
                    Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
                ),
            ),
            Variable::new("x"),
        ),
    )
    .into()
}

pub fn quadruple() -> Term {
    TyLambda::new(
        "X",
        Kind::Star,
        App::new(
            TyApp::new(
                double(),
                Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
            ),
            TyApp::new(double(), TypeVariable::new("X")),
        ),
    )
    .into()
}

#[cfg(test)]
mod examples_tests {
    use super::{double, id, quadruple, self_app};
    use common::{
        check::Typecheck,
        kinds::Kind,
        types::{Forall, Fun, TypeVariable},
    };

    #[test]
    fn ty_id() {
        let result = id().check(&mut Default::default()).unwrap();
        let expected = Forall::new(
            "X",
            Kind::Star,
            Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_dobule() {
        let result = double().check(&mut Default::default()).unwrap();
        let expected = Forall::new(
            "X",
            Kind::Star,
            Fun::new(
                Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
                Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
            ),
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_selfapp() {
        let result = self_app().check(&mut Default::default()).unwrap();
        let expected = Fun::new(
            Forall::new(
                "X",
                Kind::Star,
                Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
            ),
            Forall::new(
                "X",
                Kind::Star,
                Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
            ),
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_quadruple() {
        let result = quadruple().check(&mut Default::default()).unwrap();
        let expected = Forall::new(
            "X",
            Kind::Star,
            Fun::new(
                Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
                Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
            ),
        )
        .into();
        assert_eq!(result, expected)
    }
}
