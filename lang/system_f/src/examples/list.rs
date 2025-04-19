use super::bool::{c_bool, fls, tru};
use crate::{terms::Term, types::Type};
use common::{
    kinds::Kind,
    terms::{App, Lambda, TyApp, TyLambda, Variable},
    types::{Forall, Fun, TypeVariable},
};

pub fn list(x: Type) -> Type {
    Forall::new(
        "R",
        Kind::Star,
        Fun::new(
            Fun::new(x, Fun::new(TypeVariable::new("R"), TypeVariable::new("R"))),
            Fun::new(TypeVariable::new("R"), TypeVariable::new("R")),
        ),
    )
    .into()
}

pub fn nil() -> Term {
    TyLambda::new(
        "X",
        Kind::Star,
        TyLambda::new(
            "R",
            Kind::Star,
            Lambda::new(
                "c",
                Fun::new(
                    TypeVariable::new("X"),
                    Fun::new(TypeVariable::new("R"), TypeVariable::new("R")),
                ),
                Lambda::new("n", TypeVariable::new("R"), Variable::new("n")),
            ),
        ),
    )
    .into()
}

pub fn cons() -> Term {
    TyLambda::new(
        "X",
        Kind::Star,
        Lambda::new(
            "hd",
            TypeVariable::new("X"),
            Lambda::new(
                "tl",
                list(TypeVariable::new("X").into()),
                TyLambda::new(
                    "R",
                    Kind::Star,
                    Lambda::new(
                        "c",
                        Fun::new(
                            TypeVariable::new("X"),
                            Fun::new(TypeVariable::new("R"), TypeVariable::new("R")),
                        ),
                        Lambda::new(
                            "n",
                            TypeVariable::new("R"),
                            App::new(
                                App::new(Variable::new("c"), Variable::new("hd")),
                                App::new(
                                    App::new(
                                        TyApp::new(Variable::new("tl"), TypeVariable::new("R")),
                                        Variable::new("c"),
                                    ),
                                    Variable::new("n"),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
    .into()
}

pub fn isnil() -> Term {
    TyLambda::new(
        "X",
        Kind::Star,
        Lambda::new(
            "l",
            list(TypeVariable::new("X").into()),
            App::new(
                App::new(
                    TyApp::new(Variable::new("l"), c_bool()),
                    Lambda::new(
                        "hd",
                        TypeVariable::new("X"),
                        Lambda::new("tl", c_bool(), fls()),
                    ),
                ),
                tru(),
            ),
        ),
    )
    .into()
}

#[cfg(test)]
mod list_tests {
    use super::{c_bool, cons, isnil, list, nil};
    use common::{
        check::Typecheck,
        kinds::Kind,
        types::{Forall, Fun, TypeVariable},
    };

    #[test]
    fn ty_nil() {
        let result = nil().check(&mut Default::default()).unwrap();
        let expected = Forall::new("X", Kind::Star, list(TypeVariable::new("X").into())).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_cons() {
        let result = cons().check(&mut Default::default()).unwrap();
        let expected = Forall::new(
            "X",
            Kind::Star,
            Fun::new(
                TypeVariable::new("X"),
                Fun::new(
                    list(TypeVariable::new("X").into()),
                    list(TypeVariable::new("X").into()),
                ),
            ),
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_isnil() {
        let result = isnil().check(&mut Default::default()).unwrap();
        let expected = Forall::new(
            "X",
            Kind::Star,
            Fun::new(list(TypeVariable::new("X").into()), c_bool()),
        )
        .into();
        assert_eq!(result, expected)
    }
}
