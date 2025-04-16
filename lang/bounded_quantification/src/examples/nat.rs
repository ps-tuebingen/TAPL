use crate::{terms::Term, types::Type};
use common::{
    terms::{App, Lambda, LambdaSub, TyApp, Variable},
    types::{ForallBounded, Fun, TypeVariable},
};

pub fn c_nat() -> Type {
    ForallBounded::new_unbounded(
        "X",
        Fun::new(
            Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
            Fun::new(TypeVariable::new("X"), TypeVariable::new("X")),
        ),
    )
    .into()
}

pub fn s_nat() -> Type {
    ForallBounded::new_unbounded(
        "X",
        ForallBounded::new(
            "S",
            TypeVariable::new("X"),
            ForallBounded::new(
                "Z",
                TypeVariable::new("X"),
                Fun::new(
                    Fun::new(TypeVariable::new("X"), TypeVariable::new("S")),
                    Fun::new(TypeVariable::new("Z"), TypeVariable::new("X")),
                ),
            ),
        ),
    )
    .into()
}

pub fn ty_s_zero() -> Type {
    ForallBounded::new_unbounded(
        "X",
        ForallBounded::new(
            "S",
            TypeVariable::new("X"),
            ForallBounded::new(
                "Z",
                TypeVariable::new("X"),
                Fun::new(
                    Fun::new(TypeVariable::new("X"), TypeVariable::new("S")),
                    Fun::new(TypeVariable::new("Z"), TypeVariable::new("Z")),
                ),
            ),
        ),
    )
    .into()
}

pub fn s_zero() -> Term {
    LambdaSub::new_unbounded(
        "X",
        LambdaSub::new(
            "S",
            TypeVariable::new("X"),
            LambdaSub::new(
                "Z",
                TypeVariable::new("X"),
                Lambda::new(
                    "s",
                    Fun::new(TypeVariable::new("X"), TypeVariable::new("S")),
                    Lambda::new("z", TypeVariable::new("Z"), Variable::new("z")),
                ),
            ),
        ),
    )
    .into()
}

pub fn s_pos() -> Type {
    ForallBounded::new_unbounded(
        "X",
        ForallBounded::new(
            "S",
            TypeVariable::new("X"),
            ForallBounded::new(
                "Z",
                TypeVariable::new("X"),
                Fun::new(
                    Fun::new(TypeVariable::new("X"), TypeVariable::new("S")),
                    Fun::new(TypeVariable::new("Z"), TypeVariable::new("S")),
                ),
            ),
        ),
    )
    .into()
}

pub fn s_one() -> Term {
    LambdaSub::new_unbounded(
        "X",
        LambdaSub::new(
            "S",
            TypeVariable::new("X"),
            LambdaSub::new(
                "Z",
                TypeVariable::new("X"),
                Lambda::new(
                    "s",
                    Fun::new(TypeVariable::new("X"), TypeVariable::new("S")),
                    Lambda::new(
                        "z",
                        TypeVariable::new("Z"),
                        App::new(Variable::new("s"), Variable::new("z")),
                    ),
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
            TypeVariable::new("X"),
            LambdaSub::new(
                "Z",
                TypeVariable::new("X"),
                Lambda::new(
                    "s",
                    Fun::new(TypeVariable::new("X"), TypeVariable::new("S")),
                    Lambda::new(
                        "z",
                        TypeVariable::new("Z"),
                        App::new(
                            Variable::new("s"),
                            App::new(Variable::new("s"), Variable::new("z")),
                        ),
                    ),
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
            TypeVariable::new("X"),
            LambdaSub::new(
                "Z",
                TypeVariable::new("X"),
                Lambda::new(
                    "s",
                    Fun::new(TypeVariable::new("X"), TypeVariable::new("S")),
                    Lambda::new(
                        "z",
                        TypeVariable::new("Z"),
                        App::new(
                            Variable::new("s"),
                            App::new(
                                Variable::new("s"),
                                App::new(Variable::new("s"), Variable::new("z")),
                            ),
                        ),
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
                TypeVariable::new("X"),
                LambdaSub::new(
                    "Z",
                    TypeVariable::new("X"),
                    Lambda::new(
                        "s",
                        Fun::new(TypeVariable::new("X"), TypeVariable::new("S")),
                        Lambda::new(
                            "z",
                            TypeVariable::new("Z"),
                            App::new(
                                Variable::new("s"),
                                App::new(
                                    App::new(
                                        TyApp::new(
                                            TyApp::new(
                                                TyApp::new(
                                                    Variable::new("n"),
                                                    TypeVariable::new("X"),
                                                ),
                                                TypeVariable::new("S"),
                                            ),
                                            TypeVariable::new("Z"),
                                        ),
                                        Variable::new("s"),
                                    ),
                                    Variable::new("z"),
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
                    TypeVariable::new("X"),
                    LambdaSub::new(
                        "Z",
                        TypeVariable::new("X"),
                        Lambda::new(
                            "s",
                            Fun::new(TypeVariable::new("X"), TypeVariable::new("S")),
                            Lambda::new(
                                "z",
                                TypeVariable::new("Z"),
                                App::new(
                                    App::new(
                                        TyApp::new(
                                            TyApp::new(
                                                TyApp::new(
                                                    Variable::new("n"),
                                                    TypeVariable::new("X"),
                                                ),
                                                TypeVariable::new("S"),
                                            ),
                                            TypeVariable::new("S"),
                                        ),
                                        Variable::new("s"),
                                    ),
                                    App::new(
                                        App::new(
                                            TyApp::new(
                                                TyApp::new(
                                                    TyApp::new(
                                                        Variable::new("m"),
                                                        TypeVariable::new("X"),
                                                    ),
                                                    TypeVariable::new("S"),
                                                ),
                                                TypeVariable::new("Z"),
                                            ),
                                            Variable::new("s"),
                                        ),
                                        Variable::new("z"),
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
        let result = s_zero().check_start().unwrap();
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
        let expected = Fun::new(s_nat(), s_pos());
        assert_eq!(result, expected)
    }

    #[test]
    fn check_plus() {
        let result = s_pluspp().check(&mut Default::default()).unwrap();
        let expected = Fun::new(s_pos(), Fun::new(s_pos(), s_pos()));
        assert_eq!(result, expected)
    }
}
