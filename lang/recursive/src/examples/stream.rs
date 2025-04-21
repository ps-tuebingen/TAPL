use crate::{terms::Term, types::Type};
use common::{
    terms::{App, Fix, Fold, Fst, Lambda, Num, Pair, Snd, Succ, Unfold, Unit, Variable},
    types::{Fun, Mu, Nat, Product, TypeVariable, Unit as UnitTy},
};

pub fn ty_stream() -> Type {
    Mu::new(
        "A",
        Fun::new(
            UnitTy::new(),
            Product::new(Nat::new(), TypeVariable::new("A")),
        ),
    )
    .into()
}

pub fn hd() -> Term {
    Lambda::new(
        "s",
        ty_stream(),
        Fst::new(App::new(
            Unfold::new(ty_stream(), Variable::new("s")),
            Unit::new(),
        )),
    )
    .into()
}

pub fn tl() -> Term {
    Lambda::new(
        "s",
        ty_stream(),
        Snd::new(App::new(
            Unfold::new(ty_stream(), Variable::new("s")),
            Unit::new(),
        )),
    )
    .into()
}

pub fn upfrom0() -> Term {
    App::new(
        Fix::new(Lambda::new(
            "f",
            Fun::new(Nat::new(), ty_stream()),
            Lambda::new(
                "n",
                Nat::new(),
                Fold::new(
                    Lambda::new(
                        "_",
                        UnitTy::new(),
                        Pair::new(
                            Variable::new("n"),
                            App::new(Variable::new("f"), Succ::new(Variable::new("n"))),
                        ),
                    ),
                    ty_stream(),
                ),
            ),
        )),
        Num::new(0),
    )
    .into()
}

#[cfg(test)]
mod stream_tests {
    use super::{hd, tl, ty_stream, upfrom0};
    use common::{
        check::Typecheck,
        types::{Fun, Nat},
    };

    #[test]
    fn check_hd() {
        let result = hd().check(&mut Default::default()).unwrap();
        let expected = Fun::new(ty_stream(), Nat::new()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_tl() {
        let result = tl().check(&mut Default::default()).unwrap();
        let expected = Fun::new(ty_stream(), ty_stream()).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn check_up() {
        let result = upfrom0().check(&mut Default::default()).unwrap();
        let expected = ty_stream();
        assert_eq!(result, expected)
    }
}
