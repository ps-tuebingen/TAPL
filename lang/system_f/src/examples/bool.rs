use crate::{terms::Term, types::Type};
use common::{
    kinds::Kind,
    terms::{App, Lambda, TyApp, TyLambda, Variable},
    types::{Forall, Fun, TypeVariable},
};

pub fn c_bool() -> Type {
    Forall::new(
        "X",
        Kind::Star,
        Fun::new(
            TypeVariable::new("X"),
            Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
        ),
    )
    .into()
}

pub fn tru() -> Term {
    TyLambda::new(
        "X",
        Kind::Star,
        Lambda::new(
            "t",
            TypeVariable::new("X"),
            Lambda::new("f", TypeVariable::new("X"), Variable::new("t")),
        ),
    )
    .into()
}

pub fn fls() -> Term {
    TyLambda::new(
        "X",
        Kind::Star,
        Lambda::new(
            "x",
            TypeVariable::new("X"),
            Lambda::new("f", TypeVariable::new("X"), Variable::new("f")),
        ),
    )
    .into()
}

pub fn not() -> Term {
    Lambda::new(
        "b",
        c_bool(),
        TyLambda::new(
            "X",
            Kind::Star,
            Lambda::new(
                "t",
                TypeVariable::new("X"),
                Lambda::new(
                    "f",
                    TypeVariable::new("X"),
                    App::new(
                        App::new(
                            TyApp::new(Variable::new("b"), TypeVariable::new("X")),
                            Variable::new("f"),
                        ),
                        Variable::new("t"),
                    ),
                ),
            ),
        ),
    )
    .into()
}

#[cfg(test)]
mod bool_tests {
    use super::{c_bool, fls, not, tru};
    use common::{check::Typecheck, types::Fun};

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
        let expected = Fun::new(c_bool(), c_bool()).into();
        assert_eq!(result, expected)
    }
}
