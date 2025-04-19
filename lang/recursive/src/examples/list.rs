use crate::{terms::Term, types::Type};
use common::{
    terms::{
        variantcase::VariantPattern, False, Fold, Fst, Lambda, Num, Pair, Snd, True, Unfold, Unit,
        Variable, Variant, VariantCase,
    },
    types::{Mu, Nat, Product, TypeVariable, Unit as UnitTy, Variant as VariantTy},
    Label,
};
use std::collections::HashMap;

pub fn nat_list() -> Type {
    Mu::new(
        "X",
        VariantTy::new(HashMap::<Label, Type>::from([
            ("nil".to_owned(), UnitTy.into()),
            (
                "cons".to_owned(),
                Product::new(Nat, TypeVariable::new("X")).into(),
            ),
        ])),
    )
    .into()
}

pub fn nil() -> Term {
    Fold::new(
        Variant::new(
            "nil",
            Unit::new(),
            VariantTy::new(HashMap::<Label, Type>::from([
                ("nil".to_owned(), UnitTy.into()),
                ("cons".to_owned(), Product::new(Nat, nat_list()).into()),
            ])),
        ),
        nat_list(),
    )
    .into()
}

pub fn cons() -> Term {
    Lambda::new(
        "n",
        Nat,
        Lambda::new(
            "l",
            nat_list(),
            Fold::new(
                Variant::new(
                    "cons",
                    Pair::new(Variable::new("n"), Variable::new("l")),
                    VariantTy::new(HashMap::<Label, Type>::from([
                        ("nil".to_owned(), UnitTy.into()),
                        ("cons".to_owned(), Product::new(Nat, nat_list()).into()),
                    ])),
                ),
                nat_list(),
            ),
        ),
    )
    .into()
}

pub fn is_nil() -> Term {
    Lambda::new(
        "l",
        nat_list(),
        VariantCase::new(
            Unfold::new(nat_list(), Variable::new("l")),
            vec![
                VariantPattern::new("nil", "u", True::new()),
                VariantPattern::new("cons", "p", False::new()),
            ],
        ),
    )
    .into()
}

pub fn hd() -> Term {
    Lambda::new(
        "l",
        nat_list(),
        VariantCase::new(
            Unfold::new(nat_list(), Variable::new("l")),
            vec![
                VariantPattern::new("nil", "u", Num::new(0)),
                VariantPattern::new("cons", "p", Fst::new(Variable::new("p"))),
            ],
        ),
    )
    .into()
}

pub fn tl() -> Term {
    Lambda::new(
        "l",
        nat_list(),
        VariantCase::new(
            Unfold::new(nat_list(), Variable::new("l")),
            vec![
                VariantPattern::new("nil", "u", Variable::new("l")),
                VariantPattern::new("cons", "p", Snd::new(Variable::new("p"))),
            ],
        ),
    )
    .into()
}

#[cfg(test)]
mod list_tests {
    use super::{cons, hd, is_nil, nat_list, nil, tl};
    use common::{
        check::Typecheck,
        types::{Bool, Fun, Nat},
    };

    #[test]
    fn ty_nil() {
        let result = nil().check_start().unwrap();
        let expected = nat_list();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_cons() {
        let result = cons().check_start().unwrap();
        let expected = Fun::new(Nat, Fun::new(nat_list(), nat_list())).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_isnil() {
        let result = is_nil().check(&mut Default::default()).unwrap();
        let expected = Fun::new(nat_list(), Bool).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_hd() {
        let result = hd().check(&mut Default::default()).unwrap();
        let expected = Fun::new(nat_list(), Nat).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn ty_tl() {
        let result = tl().check(&mut Default::default()).unwrap();
        let expected = Fun::new(nat_list(), nat_list()).into();
        assert_eq!(result, expected)
    }
}
