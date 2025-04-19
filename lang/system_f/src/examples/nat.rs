use crate::{terms::Term, types::Type};
use common::{
    kinds::Kind,
    terms::{App, Lambda, TyApp, TyLambda, Variable},
    types::{Forall, Fun, TypeVariable},
};

pub fn c_nat() -> Type {
    Forall::new(
        "X",
        Kind::Star,
        Fun::new(
            Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
            Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
        ),
    )
    .into()
}

pub fn c0() -> Term {
    TyLambda::new(
        "X",
        Kind::Star,
        Lambda::new(
            "s",
            Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
            Lambda::new("z", TypeVariable::new("X"), Variable::new("z")),
        ),
    )
    .into()
}

pub fn c1() -> Term {
    TyLambda::new(
        "X",
        Kind::Star,
        Lambda::new(
            "s",
            Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
            Lambda::new(
                "z",
                TypeVariable::new("X"),
                App::new(Variable::new("s"), Variable::new("z")),
            ),
        ),
    )
    .into()
}

pub fn c2() -> Term {
    TyLambda::new(
        "X",
        Kind::Star,
        Lambda::new(
            "s",
            Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
            Lambda::new(
                "z",
                TypeVariable::new("X"),
                App::new(
                    Variable::new("s"),
                    App::new(Variable::new("s"), Variable::new("z")),
                ),
            ),
        ),
    )
    .into()
}

pub fn csucc() -> Term {
    Lambda::new(
        "n",
        c_nat(),
        TyLambda::new(
            "X",
            Kind::Star,
            Lambda::new(
                "s",
                Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
                Lambda::new(
                    "z",
                    TypeVariable::new("X"),
                    App::new(
                        Variable::new("s"),
                        App::new(
                            App::new(
                                TyApp::new(Variable::new("n"), TypeVariable::new("X")),
                                Variable::new("s"),
                            ),
                            Variable::new("z"),
                        ),
                    ),
                ),
            ),
        ),
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
                App::new(TyApp::new(Variable::new("m"), c_nat()), csucc()),
                Variable::new("n"),
            ),
        ),
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
                Kind::Star,
                Lambda::new(
                    "s",
                    Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
                    App::new(
                        TyApp::new(Variable::new("n"), TypeVariable::new("X")),
                        App::new(
                            TyApp::new(Variable::new("m"), TypeVariable::new("X")),
                            Variable::new("s"),
                        ),
                    ),
                ),
            ),
        ),
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
                Kind::Star,
                App::new(
                    TyApp::new(
                        Variable::new("n"),
                        Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
                    ),
                    TyApp::new(Variable::new("m"), TypeVariable::new("X")),
                ),
            ),
        ),
    )
    .into()
}

#[cfg(test)]
mod nat_tests {
    use super::{c0, c1, c2, c_nat, cexp, cplus, csucc, ctimes};
    use common::{check::Typecheck, types::Fun};

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
        let expected = Fun::new(c_nat(), c_nat()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_cplus() {
        let result = cplus().check(&mut Default::default()).unwrap();
        let expected = Fun::new(c_nat(), Fun::new(c_nat(), c_nat())).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_ctimes() {
        let result = ctimes().check(&mut Default::default()).unwrap();
        let expected = Fun::new(c_nat(), Fun::new(c_nat(), c_nat())).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_cexp() {
        let result = cexp().check(&mut Default::default()).unwrap();
        let expected = Fun::new(c_nat(), Fun::new(c_nat(), c_nat())).into();
        assert_eq!(result, expected)
    }
}
