use crate::{terms::Term, types::Type};
use common::{
    terms::{App, Lambda, LambdaSub, TyApp, Variable},
    types::{ForallBounded, Fun, TypeVariable},
};

pub fn ty_pair(ty1: Type, ty2: Type) -> Type {
    let var = ty1.fresh_tyvar(vec![&ty2]);
    ForallBounded::new_unbounded(
        &var,
        Fun::new(
            Fun::new(ty1, Fun::new(ty2, TypeVariable::new(&var))),
            TypeVariable::new(&var),
        ),
    )
    .into()
}

pub fn pair() -> Term {
    LambdaSub::new_unbounded(
        "X",
        LambdaSub::new_unbounded(
            "Y",
            Lambda::new(
                "x",
                TypeVariable::new("X"),
                Lambda::new(
                    "y",
                    TypeVariable::new("Y"),
                    LambdaSub::new_unbounded(
                        "R",
                        Lambda::new(
                            "p",
                            Fun::new(
                                TypeVariable::new("X"),
                                Fun::new(TypeVariable::new("Y"), TypeVariable::new("R")),
                            ),
                            App::new(
                                App::new(Variable::new("p"), Variable::new("x")),
                                Variable::new("y"),
                            ),
                        ),
                    ),
                ),
            ),
        ),
    )
    .into()
}

pub fn fst() -> Term {
    LambdaSub::new_unbounded(
        "X",
        LambdaSub::new_unbounded(
            "Y",
            Lambda::new(
                "p",
                ty_pair(TypeVariable::new("X").into(), TypeVariable::new("Y").into()),
                App::new(
                    TyApp::new(Variable::new("p"), TypeVariable::new("X")),
                    Lambda::new(
                        "x",
                        TypeVariable::new("X"),
                        Lambda::new("y", TypeVariable::new("Y"), Variable::new("x")),
                    ),
                ),
            ),
        ),
    )
    .into()
}

pub fn snd() -> Term {
    LambdaSub::new_unbounded(
        "X",
        LambdaSub::new_unbounded(
            "Y",
            Lambda::new(
                "p",
                ty_pair(TypeVariable::new("X").into(), TypeVariable::new("Y").into()),
                App::new(
                    TyApp::new(Variable::new("p"), TypeVariable::new("Y")),
                    Lambda::new(
                        "x",
                        TypeVariable::new("X"),
                        Lambda::new("y", TypeVariable::new("Y"), Variable::new("y")),
                    ),
                ),
            ),
        ),
    )
    .into()
}

#[cfg(test)]
mod pair_tests {
    use super::{fst, pair, snd, ty_pair};
    use crate::types::Type;
    use common::Typecheck;

    #[test]
    fn check_pair() {
        let result = pair().check(&mut Default::default()).unwrap();
        let expected = Type::forall_unbounded(
            "X",
            Type::forall_unbounded(
                "Y",
                Fun::new(
                    TypeVariable::new("X"),
                    Fun::new(
                        TypeVariable::new("Y")(),
                        ty_pair(TypeVariable::new("X"), TypeVariable::new("Y")())
                            .rename("X0".to_owned(), "R".to_owned()),
                    ),
                ),
            ),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn check_fst() {
        let result = fst().check(&mut Default::default()).unwrap();
        let expected = Type::forall_unbounded(
            "X",
            Type::forall_unbounded(
                "Y",
                Fun::new(
                    ty_pair(TypeVariable::new("X"), TypeVariable::new("Y")()),
                    "X".into(),
                ),
            ),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn check_snd() {
        let result = snd()
            .check(&mut Default::default())
            .map_err(|err| err.to_string())
            .unwrap();
        let expected = Type::forall_unbounded(
            "X",
            Type::forall_unbounded(
                "Y",
                Fun::new(
                    ty_pair(TypeVariable::new("X"), TypeVariable::new("Y")()),
                    TypeVariable::new("Y")(),
                ),
            ),
        );
        assert_eq!(result, expected)
    }
}
