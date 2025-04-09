use super::nat::plus;
use crate::{
    terms::{App, Fix, Fold, Fst, Lambda, Let, Pair, Snd, Term, Unfold, Zero},
    types::Type,
};

pub fn ty_process() -> Type {
    Type::mu("A", Type::fun(Type::Nat, Type::pair(Type::Nat, "A".into())))
}

pub fn proc() -> Term {
    App::new(
        Fix::new(
            Lambda::new(
                "f",
                Type::fun(Type::Nat, ty_process()),
                Lambda::new(
                    "acc",
                    Type::Nat,
                    Fold::new(
                        Lambda::new(
                            "n",
                            Type::Nat,
                            Let::new(
                                "newacc",
                                App::new(App::new(plus(), "acc".into()).into(), "n".into()).into(),
                                Pair::new(
                                    "newacc".into(),
                                    App::new("f".into(), "newacc".into()).into(),
                                )
                                .into(),
                            )
                            .into(),
                        )
                        .into(),
                        ty_process(),
                    )
                    .into(),
                )
                .into(),
            )
            .into(),
        )
        .into(),
        Zero.into(),
    )
    .into()
}

pub fn curr() -> Term {
    Lambda::new(
        "s",
        ty_process(),
        Fst::new(App::new(Unfold::new("s".into(), ty_process()).into(), Zero.into()).into()).into(),
    )
    .into()
}

pub fn send() -> Term {
    Lambda::new(
        "n",
        Type::Nat,
        Lambda::new(
            "s",
            ty_process(),
            Snd::new(App::new(Unfold::new("s".into(), ty_process()).into(), "n".into()).into())
                .into(),
        )
        .into(),
    )
    .into()
}

#[cfg(test)]
mod process_tests {
    use super::{curr, proc, send, ty_process};
    use crate::types::Type;
    use common::Typecheck;

    #[test]
    fn check_proc() {
        let result = proc().check(&mut Default::default()).unwrap();
        let expected = ty_process();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_curr() {
        let result = curr().check(&mut Default::default()).unwrap();
        let expected = Type::fun(ty_process(), Type::Nat);
        assert_eq!(result, expected)
    }

    #[test]
    fn check_send() {
        let result = send().check(&mut Default::default()).unwrap();
        let expected = Type::fun(Type::Nat, Type::fun(ty_process(), ty_process()));
        assert_eq!(result, expected)
    }
}
