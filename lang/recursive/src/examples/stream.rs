use crate::{
    terms::{App, Fix, Fold, Fst, Lambda, Pair, Snd, Succ, Term, Unfold, Zero},
    types::Type,
};

pub fn ty_stream() -> Type {
    Type::mu(
        "A",
        Type::fun(Type::Unit, Type::pair(Type::Nat, "A".into())),
    )
}

pub fn hd() -> Term {
    Lambda::new(
        "s",
        ty_stream(),
        Fst::new(App::new(Unfold::new("s".into(), ty_stream()).into(), Term::Unit).into()).into(),
    )
    .into()
}

pub fn tl() -> Term {
    Lambda::new(
        "s",
        ty_stream(),
        Snd::new(App::new(Unfold::new("s".into(), ty_stream()).into(), Term::Unit).into()).into(),
    )
    .into()
}

pub fn upfrom0() -> Term {
    App::new(
        Fix::new(
            Lambda::new(
                "f",
                Type::fun(Type::Nat, ty_stream()),
                Lambda::new(
                    "n",
                    Type::Nat,
                    Fold::new(
                        Lambda::new(
                            "_",
                            Type::Unit,
                            Pair::new(
                                "n".into(),
                                App::new("f".into(), Succ::new("n".into()).into()).into(),
                            )
                            .into(),
                        )
                        .into(),
                        ty_stream(),
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

#[cfg(test)]
mod stream_tests {
    use super::{hd, tl, ty_stream, upfrom0};
    use crate::{check::Check, types::Type};

    #[test]
    fn check_hd() {
        let result = hd().check(&mut Default::default()).unwrap();
        let expected = Type::fun(ty_stream(), Type::Nat);
        assert_eq!(result, expected)
    }

    #[test]
    fn check_tl() {
        let result = tl().check(&mut Default::default()).unwrap();
        let expected = Type::fun(ty_stream(), ty_stream());
        assert_eq!(result, expected)
    }

    #[test]
    fn check_up() {
        let result = upfrom0().check(&mut Default::default()).unwrap();
        let expected = ty_stream();
        assert_eq!(result, expected)
    }
}
