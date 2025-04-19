use crate::terms::Term;
use common::{
    terms::{App, Fix, If, IsZero, Lambda, Pred, Variable},
    types::{Fun, Nat},
};

pub fn plus() -> Term {
    Fix::new(Lambda::new(
        "f",
        Fun::new(Nat, Fun::new(Nat, Nat)),
        Lambda::new(
            "m",
            Nat,
            Lambda::new(
                "n",
                Nat,
                If::new(
                    IsZero::new(Variable::new("m")),
                    Variable::new("n"),
                    App::new(
                        App::new(Variable::new("f"), Pred::new(Variable::new("m"))),
                        Variable::new("n"),
                    ),
                ),
            ),
        ),
    ))
    .into()
}

#[cfg(test)]
mod nat_tests {
    use super::plus;
    use common::{
        check::Typecheck,
        types::{Fun, Nat},
    };

    #[test]
    fn check_plus() {
        let result = plus().check(&mut Default::default()).unwrap();
        let expected = Fun::new(Nat, Fun::new(Nat, Nat)).into();
        assert_eq!(result, expected)
    }
}
