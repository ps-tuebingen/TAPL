use super::{
    named_terms::Term as NamedTerm, nameless_terms::Term as NamelessTerm,
    naming_context::NamingContext,
};

pub fn restore_names(t: NamelessTerm) -> NamedTerm {
    let mut ctx = NamingContext::default();
    restore_with_ctx(t, &mut ctx)
}

pub fn restore_with_ctx(t: NamelessTerm, ctx: &mut NamingContext) -> NamedTerm {
    match t {
        NamelessTerm::Var(ind) => {
            let var = ctx.mappings.iter().find(|(_, i)| *i == ind);
            match var {
                Some(v) => NamedTerm::Var(v.0.clone()),
                None => {
                    let new_ind = match ctx.max_ind() {
                        None => 0,
                        Some(i) => i + 1,
                    };
                    let new_var = format!("x{new_ind}");
                    NamedTerm::Var(new_var)
                }
            }
        }
        NamelessTerm::Lambda(t) => {
            let new_ind = match ctx.max_ind() {
                None => 0,
                Some(i) => i + 1,
            };
            let new_var = format!("x{new_ind}");
            ctx.add_var(new_var.clone());
            let inner = restore_with_ctx(*t, ctx);
            NamedTerm::Lambda(new_var, Box::new(inner))
        }
        NamelessTerm::App(t1, t2) => {
            let t1_res = restore_with_ctx(*t1, &mut ctx.clone());
            let t2_res = restore_with_ctx(*t2, ctx);
            NamedTerm::App(Box::new(t1_res), Box::new(t2_res))
        }
    }
}

#[cfg(test)]
mod restore_names_tests {
    use super::{restore_names, NamedTerm, NamelessTerm};

    #[test]
    fn restore_c0() {
        let result = restore_names(NamelessTerm::lam(NamelessTerm::lam(NamelessTerm::Var(0))));
        let expected = NamedTerm::lam("x0", NamedTerm::lam("x1", NamedTerm::var("x1")));
        assert_eq!(result, expected)
    }

    #[test]
    fn restore_c2() {
        let result = restore_names(NamelessTerm::lam(NamelessTerm::lam(NamelessTerm::app(
            NamelessTerm::Var(1),
            NamelessTerm::app(NamelessTerm::Var(1), NamelessTerm::Var(0)),
        ))));
        let expected = NamedTerm::lam(
            "x0",
            NamedTerm::lam(
                "x1",
                NamedTerm::app(
                    NamedTerm::var("x0"),
                    NamedTerm::app(NamedTerm::var("x0"), NamedTerm::var("x1")),
                ),
            ),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn restore_plus() {
        let result = restore_names(NamelessTerm::lam(NamelessTerm::lam(NamelessTerm::lam(
            NamelessTerm::lam(NamelessTerm::app(
                NamelessTerm::app(NamelessTerm::Var(3), NamelessTerm::Var(1)),
                NamelessTerm::app(
                    NamelessTerm::app(NamelessTerm::Var(2), NamelessTerm::Var(0)),
                    NamelessTerm::Var(1),
                ),
            )),
        ))));
        let expected = NamedTerm::lam(
            "x0",
            NamedTerm::lam(
                "x1",
                NamedTerm::lam(
                    "x2",
                    NamedTerm::lam(
                        "x3",
                        NamedTerm::app(
                            NamedTerm::app(NamedTerm::var("x0"), NamedTerm::var("x2")),
                            NamedTerm::app(
                                NamedTerm::app(NamedTerm::var("x1"), NamedTerm::var("x3")),
                                NamedTerm::var("x2"),
                            ),
                        ),
                    ),
                ),
            ),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn restore_fix() {
        let result = restore_names(NamelessTerm::lam(NamelessTerm::app(
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
        )));
        let expected = NamedTerm::lam(
            "x0",
            NamedTerm::app(
                NamedTerm::lam(
                    "x1",
                    NamedTerm::app(
                        NamedTerm::var("x0"),
                        NamedTerm::lam(
                            "x2",
                            NamedTerm::app(
                                NamedTerm::app(NamedTerm::var("x1"), NamedTerm::var("x1")),
                                NamedTerm::var("x2"),
                            ),
                        ),
                    ),
                ),
                NamedTerm::lam(
                    "x1",
                    NamedTerm::app(
                        NamedTerm::var("x0"),
                        NamedTerm::lam(
                            "x2",
                            NamedTerm::app(
                                NamedTerm::app(NamedTerm::var("x1"), NamedTerm::var("x1")),
                                NamedTerm::var("x2"),
                            ),
                        ),
                    ),
                ),
            ),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn restore_foo() {
        let result = restore_names(NamelessTerm::app(
            NamelessTerm::lam(NamelessTerm::lam(NamelessTerm::Var(0))),
            NamelessTerm::lam(NamelessTerm::Var(0)),
        ));
        let expected = NamedTerm::app(
            NamedTerm::lam("x0", NamedTerm::lam("x1", NamedTerm::var("x1"))),
            NamedTerm::lam("x0", NamedTerm::var("x0")),
        );
        assert_eq!(result, expected)
    }
}
