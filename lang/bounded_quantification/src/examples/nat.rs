use crate::{
    syntax::{App, Lambda, LambdaSub, Term, TyApp},
    types::Type,
};

pub fn c_nat() -> Type {
    Type::forall_unbounded(
        "X",
        Type::fun(
            Type::fun("X".into(), "X".into()),
            Type::fun("X".into(), "X".into()),
        ),
    )
}

pub fn s_nat() -> Type {
    Type::forall_unbounded(
        "X",
        Type::forall(
            "S",
            "X".into(),
            Type::forall(
                "Z",
                "X".into(),
                Type::fun(
                    Type::fun("X".into(), "S".into()),
                    Type::fun("Z".into(), "X".into()),
                ),
            ),
        ),
    )
}

pub fn ty_s_zero() -> Type {
    Type::forall_unbounded(
        "X",
        Type::forall(
            "S",
            "X".into(),
            Type::forall(
                "Z",
                "X".into(),
                Type::fun(
                    Type::fun("X".into(), "S".into()),
                    Type::fun("Z".into(), "Z".into()),
                ),
            ),
        ),
    )
}

pub fn s_zero() -> Term {
    LambdaSub::new_unbounded(
        "X",
        LambdaSub::new(
            "S",
            "X".into(),
            LambdaSub::new(
                "Z",
                "X".into(),
                Lambda::new(
                    "s",
                    Type::fun("X".into(), "S".into()),
                    Lambda::new("z", "Z".into(), "z"),
                ),
            ),
        ),
    )
    .into()
}

pub fn s_pos() -> Type {
    Type::forall_unbounded(
        "X",
        Type::forall(
            "S",
            "X".into(),
            Type::forall(
                "Z",
                "X".into(),
                Type::fun(
                    Type::fun("X".into(), "S".into()),
                    Type::fun("Z".into(), "S".into()),
                ),
            ),
        ),
    )
}

pub fn s_one() -> Term {
    LambdaSub::new_unbounded(
        "X",
        LambdaSub::new(
            "S",
            "X".into(),
            LambdaSub::new(
                "Z",
                "X".into(),
                Lambda::new(
                    "s",
                    Type::fun("X".into(), "S".into()),
                    Lambda::new("z", "Z".into(), App::new("s", "z")),
                ),
            ),
        ),
    )
    .into()
}

pub fn s_two() -> Term {
    LambdaSub::new_unbounded(
        "X",
        LambdaSub::new(
            "S",
            "X".into(),
            LambdaSub::new(
                "Z",
                "X".into(),
                Lambda::new(
                    "s",
                    Type::fun("X".into(), "S".into()),
                    Lambda::new("z", "Z".into(), App::new("s", App::new("s", "z"))),
                ),
            ),
        ),
    )
    .into()
}

pub fn s_three() -> Term {
    LambdaSub::new_unbounded(
        "X",
        LambdaSub::new(
            "S",
            "X".into(),
            LambdaSub::new(
                "Z",
                "X".into(),
                Lambda::new(
                    "s",
                    Type::fun("X".into(), "S".into()),
                    Lambda::new(
                        "z",
                        "Z".into(),
                        App::new("s", App::new("s", App::new("s", "z"))),
                    ),
                ),
            ),
        ),
    )
    .into()
}

pub fn s_succ() -> Term {
    Lambda::new(
        "n",
        s_nat(),
        LambdaSub::new_unbounded(
            "X",
            LambdaSub::new(
                "S",
                "X".into(),
                LambdaSub::new(
                    "Z",
                    "X".into(),
                    Lambda::new(
                        "s",
                        Type::fun("X".into(), "S".into()),
                        Lambda::new(
                            "z",
                            "Z".into(),
                            App::new(
                                "s",
                                App::new(
                                    App::new(
                                        TyApp::new(
                                            TyApp::new(TyApp::new("n", "X".into()), "S".into()),
                                            "Z".into(),
                                        ),
                                        "s",
                                    ),
                                    "z",
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

pub fn s_pluspp() -> Term {
    Lambda::new(
        "n",
        s_pos(),
        Lambda::new(
            "m",
            s_pos(),
            LambdaSub::new_unbounded(
                "X",
                LambdaSub::new(
                    "S",
                    "X".into(),
                    LambdaSub::new(
                        "Z",
                        "X".into(),
                        Lambda::new(
                            "s",
                            Type::fun("X".into(), "S".into()),
                            Lambda::new(
                                "z",
                                "Z".into(),
                                App::new(
                                    App::new(
                                        TyApp::new(
                                            TyApp::new(TyApp::new("n", "X".into()), "S".into()),
                                            "S".into(),
                                        ),
                                        "s",
                                    ),
                                    App::new(
                                        App::new(
                                            TyApp::new(
                                                TyApp::new(TyApp::new("m", "X".into()), "S".into()),
                                                "Z".into(),
                                            ),
                                            "s",
                                        ),
                                        "z",
                                    ),
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

#[cfg(test)]
mod nat_tests {
    use super::{s_nat, s_one, s_pluspp, s_pos, s_succ, s_three, s_two, s_zero, ty_s_zero};
    use crate::types::Type;
    use common::Typecheck;

    #[test]
    fn check_zero() {
        let result = s_zero().check(&mut Default::default()).unwrap();
        let expected = ty_s_zero();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_one() {
        let result = s_one().check(&mut Default::default()).unwrap();
        let expected = s_pos();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_two() {
        let result = s_two().check(&mut Default::default()).unwrap();
        let expected = s_pos();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_three() {
        let result = s_three().check(&mut Default::default()).unwrap();
        let expected = s_pos();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_succ() {
        let result = s_succ().check(&mut Default::default()).unwrap();
        let expected = Type::fun(s_nat(), s_pos());
        assert_eq!(result, expected)
    }

    #[test]
    fn check_plus() {
        let result = s_pluspp().check(&mut Default::default()).unwrap();
        let expected = Type::fun(s_pos(), Type::fun(s_pos(), s_pos()));
        assert_eq!(result, expected)
    }
}
