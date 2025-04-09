use crate::{
    terms::{
        False, Fold, Fst, Lambda, Pair, Snd, Term, True, Unfold, Variant, VariantCase,
        VariantPattern, Zero,
    },
    types::Type,
};

pub fn nat_list() -> Type {
    Type::mu(
        "X",
        Type::variant(vec![
            ("nil", Type::Unit),
            ("cons", Type::pair(Type::Nat, "X".into())),
        ]),
    )
}

pub fn nil() -> Term {
    Fold::new(
        Variant::new(
            "nil",
            Term::Unit,
            Type::variant(vec![
                ("nil", Type::Unit),
                ("cons", Type::pair(Type::Nat, nat_list())),
            ]),
        )
        .into(),
        nat_list(),
    )
    .into()
}

pub fn cons() -> Term {
    Lambda::new(
        "n",
        Type::Nat,
        Lambda::new(
            "l",
            nat_list(),
            Fold::new(
                Variant::new(
                    "cons",
                    Pair::new("n".into(), "l".into()).into(),
                    Type::variant(vec![
                        ("nil", Type::Unit),
                        ("cons", Type::pair(Type::Nat, nat_list())),
                    ]),
                )
                .into(),
                nat_list(),
            )
            .into(),
        )
        .into(),
    )
    .into()
}

pub fn is_nil() -> Term {
    Lambda::new(
        "l",
        nat_list(),
        VariantCase::new(
            Unfold::new("l".into(), nat_list()).into(),
            vec![
                VariantPattern::new("nil", "u", True.into()),
                VariantPattern::new("cons", "p", False.into()),
            ],
        )
        .into(),
    )
    .into()
}

pub fn hd() -> Term {
    Lambda::new(
        "l",
        nat_list(),
        VariantCase::new(
            Unfold::new("l".into(), nat_list()).into(),
            vec![
                VariantPattern::new("nil", "u", Zero.into()),
                VariantPattern::new("cons", "p", Fst::new("p".into()).into()),
            ],
        )
        .into(),
    )
    .into()
}

pub fn tl() -> Term {
    Lambda::new(
        "l",
        nat_list(),
        VariantCase::new(
            Unfold::new("l".into(), nat_list()).into(),
            vec![
                VariantPattern::new("nil", "u", "l".into()),
                VariantPattern::new("cons", "p", Snd::new("p".into()).into()),
            ],
        )
        .into(),
    )
    .into()
}

#[cfg(test)]
mod list_tests {
    use super::{cons, hd, is_nil, nat_list, nil, tl};
    use crate::types::Type;
    use common::Typecheck;

    #[test]
    fn ty_nil() {
        let result = nil().check(&mut Default::default()).unwrap();
        let expected = nat_list();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_cons() {
        let result = cons().check(&mut Default::default()).unwrap();
        let expected = Type::fun(Type::Nat, Type::fun(nat_list(), nat_list()));
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_isnil() {
        let result = is_nil().check(&mut Default::default()).unwrap();
        let expected = Type::fun(nat_list(), Type::Bool);
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_hd() {
        let result = hd().check(&mut Default::default()).unwrap();
        let expected = Type::fun(nat_list(), Type::Nat);
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_tl() {
        let result = tl().check(&mut Default::default()).unwrap();
        let expected = Type::fun(nat_list(), nat_list());
        assert_eq!(result, expected)
    }
}
