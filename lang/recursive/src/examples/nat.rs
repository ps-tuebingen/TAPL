use crate::{
    terms::{App, Fix, If, IsZero, Lambda, Pred, Term},
    types::Type,
};

pub fn plus() -> Term {
    Fix::new(
        Lambda::new(
            "f",
            Type::fun(Type::Nat, Type::fun(Type::Nat, Type::Nat)),
            Lambda::new(
                "m",
                Type::Nat,
                Lambda::new(
                    "n",
                    Type::Nat,
                    If::new(
                        IsZero::new("m".into()).into(),
                        "n".into(),
                        App::new(
                            App::new("f".into(), Pred::new("m".into()).into()).into(),
                            "n".into(),
                        )
                        .into(),
                    )
                    .into(),
                )
                .into(),
            )
            .into(),
        )
        .into(),
    )
    .into()
}

#[cfg(test)]
mod nat_tests {
    use super::plus;
    use crate::types::Type;
    use common::Typecheck;

    #[test]
    fn check_plus() {
        let result = plus().check(&mut Default::default()).unwrap();
        let expected = Type::fun(Type::Nat, Type::fun(Type::Nat, Type::Nat));
        assert_eq!(result, expected)
    }
}
