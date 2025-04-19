use super::nat::plus;
use crate::{terms::Term, types::Type};
use common::{
    terms::{App, Fix, Fold, Fst, Lambda, Let, Num, Pair, Snd, Unfold, Variable},
    types::{Fun, Mu, Nat, Product, TypeVariable},
};

pub fn ty_process() -> Type {
    Mu::new(
        "A",
        Fun::new(Nat, Product::new(Nat, TypeVariable::new("A"))),
    )
    .into()
}

pub fn proc() -> Term {
    App::new(
        Fix::new(Lambda::new(
            "f",
            Fun::new(Nat, ty_process()),
            Lambda::new(
                "acc",
                Nat,
                Fold::new(
                    Lambda::new(
                        "n",
                        Nat,
                        Let::new(
                            "newacc",
                            App::new(App::new(plus(), Variable::new("acc")), Variable::new("n")),
                            Pair::new(
                                Variable::new("newacc"),
                                App::new(Variable::new("f"), Variable::new("newacc")),
                            ),
                        ),
                    ),
                    ty_process(),
                ),
            ),
        )),
        Num::new(0),
    )
    .into()
}

pub fn curr() -> Term {
    Lambda::new(
        "s",
        ty_process(),
        Fst::new(App::new(
            Unfold::new(ty_process(), Variable::new("s")),
            Num::new(0),
        )),
    )
    .into()
}

pub fn send() -> Term {
    Lambda::new(
        "n",
        Nat,
        Lambda::new(
            "s",
            ty_process(),
            Snd::new(App::new(
                Unfold::new(ty_process(), Variable::new("s")),
                Variable::new("n"),
            )),
        ),
    )
    .into()
}

#[cfg(test)]
mod process_tests {
    use super::{curr, proc, send, ty_process};
    use common::{
        check::Typecheck,
        types::{Fun, Nat},
    };

    #[test]
    fn check_proc() {
        let result = proc().check(&mut Default::default()).unwrap();
        let expected = ty_process();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_curr() {
        let result = curr().check(&mut Default::default()).unwrap();
        let expected = Fun::new(ty_process(), Nat).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_send() {
        let result = send().check(&mut Default::default()).unwrap();
        let expected = Fun::new(Nat, Fun::new(ty_process(), ty_process())).into();
        assert_eq!(result, expected)
    }
}
