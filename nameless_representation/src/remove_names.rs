use super::{
    named_terms::Term as NamedTerm, nameless_terms::Term as NamelessTerm,
    naming_context::NamingContext,
};

pub fn remove_names(t: NamedTerm) -> NamelessTerm {
    let mut ctx = NamingContext::from(t.clone());
    remove_with_ctx(t, &mut ctx).unwrap()
}

fn remove_with_ctx(t: NamedTerm, ctx: &mut NamingContext) -> Result<NamelessTerm, String> {
    match t {
        NamedTerm::Var(v) => {
            let ind = ctx
                .mappings
                .iter()
                .find(|(var, _)| *var == v)
                .ok_or(format!("Could not find free variable {v}"))?;
            Ok(NamelessTerm::Var(ind.1))
        }
        NamedTerm::Lambda(var, t) => {
            ctx.add_var(var);
            let inner = remove_with_ctx(*t, ctx)?;
            Ok(NamelessTerm::Lambda(Box::new(inner)))
        }
        NamedTerm::App(t1, t2) => {
            let t1_nameless = remove_with_ctx(*t1, &mut ctx.clone())?;
            let t2_nameless = remove_with_ctx(*t2, ctx)?;
            Ok(NamelessTerm::App(
                Box::new(t1_nameless),
                Box::new(t2_nameless),
            ))
        }
    }
}

#[cfg(test)]
mod remove_names_tests {
    use super::{remove_names, NamedTerm, NamelessTerm};

    #[test]
    fn remove_c0() {
        let result = remove_names(NamedTerm::lam(
            "s",
            NamedTerm::lam("s", NamedTerm::var("s")),
        ));
        let expected = NamelessTerm::lam(NamelessTerm::lam(NamelessTerm::Var(0)));
        assert_eq!(result, expected)
    }

    #[test]
    fn remove_c2() {
        let result = remove_names(NamedTerm::lam(
            "s",
            NamedTerm::lam(
                "z",
                NamedTerm::app(
                    NamedTerm::var("s"),
                    NamedTerm::app(NamedTerm::var("s"), NamedTerm::var("z")),
                ),
            ),
        ));
        let expected = NamelessTerm::lam(NamelessTerm::lam(NamelessTerm::app(
            NamelessTerm::Var(1),
            NamelessTerm::app(NamelessTerm::Var(1), NamelessTerm::Var(0)),
        )));
        assert_eq!(result, expected)
    }

    #[test]
    fn remove_plus() {
        let result = remove_names(NamedTerm::lam(
            "m",
            NamedTerm::lam(
                "n",
                NamedTerm::lam(
                    "s",
                    NamedTerm::lam(
                        "z",
                        NamedTerm::app(
                            NamedTerm::app(NamedTerm::var("m"), NamedTerm::var("s")),
                            NamedTerm::app(
                                NamedTerm::app(NamedTerm::var("n"), NamedTerm::var("z")),
                                NamedTerm::var("s"),
                            ),
                        ),
                    ),
                ),
            ),
        ));
        let expected = NamelessTerm::lam(NamelessTerm::lam(NamelessTerm::lam(NamelessTerm::lam(
            NamelessTerm::app(
                NamelessTerm::app(NamelessTerm::Var(3), NamelessTerm::Var(1)),
                NamelessTerm::app(
                    NamelessTerm::app(NamelessTerm::Var(2), NamelessTerm::Var(0)),
                    NamelessTerm::Var(1),
                ),
            ),
        ))));
        assert_eq!(result, expected)
    }

    #[test]
    fn remove_fix() {
        let result = remove_names(NamedTerm::lam(
            "f",
            NamedTerm::app(
                NamedTerm::lam(
                    "x",
                    NamedTerm::app(
                        NamedTerm::var("f"),
                        NamedTerm::lam(
                            "y",
                            NamedTerm::app(
                                NamedTerm::app(NamedTerm::var("x"), NamedTerm::var("x")),
                                NamedTerm::var("y"),
                            ),
                        ),
                    ),
                ),
                NamedTerm::lam(
                    "x",
                    NamedTerm::app(
                        NamedTerm::var("f"),
                        NamedTerm::lam(
                            "y",
                            NamedTerm::app(
                                NamedTerm::app(NamedTerm::var("x"), NamedTerm::var("x")),
                                NamedTerm::var("y"),
                            ),
                        ),
                    ),
                ),
            ),
        ));

        let expected = NamelessTerm::lam(NamelessTerm::app(
            NamelessTerm::lam(NamelessTerm::app(
                NamelessTerm::Var(1),
                NamelessTerm::lam(NamelessTerm::app(
                    NamelessTerm::app(NamelessTerm::Var(1), NamelessTerm::Var(1)),
                    NamelessTerm::Var(0),
                )),
            )),
            NamelessTerm::lam(NamelessTerm::app(
                NamelessTerm::Var(1),
                NamelessTerm::lam(NamelessTerm::app(
                    NamelessTerm::app(NamelessTerm::Var(1), NamelessTerm::Var(1)),
                    NamelessTerm::Var(0),
                )),
            )),
        ));
        assert_eq!(result, expected)
    }

    #[test]
    fn remove_foo() {
        let result = remove_names(NamedTerm::app(
            NamedTerm::lam("x", NamedTerm::lam("x", NamedTerm::var("x"))),
            NamedTerm::lam("x", NamedTerm::var("x")),
        ));
        let expected = NamelessTerm::app(
            NamelessTerm::lam(NamelessTerm::lam(NamelessTerm::Var(0))),
            NamelessTerm::lam(NamelessTerm::Var(0)),
        );
        assert_eq!(result, expected)
    }
}
