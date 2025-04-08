use crate::{
    syntax::{App, Lambda, LambdaSub, Term, TyApp},
    types::Type,
};

pub fn ty_pair(ty1: Type, ty2: Type) -> Type {
    let var = ty1.fresh_tyvar(vec![&ty2]);
    Type::forall_unbounded(
        var.as_str(),
        Type::fun(
            Type::fun(ty1, Type::fun(ty2, (var.as_str()).into())),
            (var.as_str()).into(),
        ),
    )
}

pub fn pair() -> Term {
    LambdaSub::new_unbounded(
        "X",
        LambdaSub::new_unbounded(
            "Y",
            Lambda::new(
                "x",
                "X".into(),
                Lambda::new(
                    "y",
                    "Y".into(),
                    LambdaSub::new_unbounded(
                        "R",
                        Lambda::new(
                            "p",
                            Type::fun("X".into(), Type::fun("Y".into(), "R".into())),
                            App::new(App::new("p", "x"), "y"),
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
                ty_pair("X".into(), "Y".into()),
                App::new(
                    TyApp::new("p", "X".into()),
                    Lambda::new("x", "X".into(), Lambda::new("y", "Y".into(), "x")),
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
                ty_pair("X".into(), "Y".into()),
                App::new(
                    TyApp::new("p", "Y".into()),
                    Lambda::new("x", "X".into(), Lambda::new("y", "Y".into(), "y")),
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
                Type::fun(
                    "X".into(),
                    Type::fun(
                        "Y".into(),
                        ty_pair("X".into(), "Y".into()).rename("X0".to_owned(), "R".to_owned()),
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
            Type::forall_unbounded("Y", Type::fun(ty_pair("X".into(), "Y".into()), "X".into())),
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
            Type::forall_unbounded("Y", Type::fun(ty_pair("X".into(), "Y".into()), "Y".into())),
        );
        assert_eq!(result, expected)
    }
}
