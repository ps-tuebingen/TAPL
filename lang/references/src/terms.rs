use super::types::Type;
use std::{collections::HashSet, fmt};

pub type Var = String;
pub type Loc = usize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Var),
    Const(i64),
    Lambda {
        var: Var,
        annot: Type,
        body: Box<Term>,
    },
    App {
        fun: Box<Term>,
        arg: Box<Term>,
    },
    Unit,
    Ref(Box<Term>),
    Deref(Box<Term>),
    Assign {
        to: Box<Term>,
        body: Box<Term>,
    },
    Loc(Loc),
    Let {
        var: Var,
        bound_term: Box<Term>,
        in_term: Box<Term>,
    },
}

impl Term {
    pub fn seq(self, t2: Term) -> Term {
        let used = self.free_vars();
        let mut fresh = 0;
        while used.contains(&format!("x{fresh}")) {
            fresh += 1;
        }
        let var = format!("x{fresh}");

        Term::App {
            fun: Box::new(Term::Lambda {
                var,
                annot: Type::Unit,
                body: Box::new(t2),
            }),
            arg: Box::new(self),
        }
    }

    pub fn lam(v: &str, annot: Type, body: Term) -> Term {
        Term::Lambda {
            var: v.to_owned(),
            annot,
            body: Box::new(body),
        }
    }

    pub fn app(t1: Term, t2: Term) -> Term {
        Term::App {
            fun: Box::new(t1),
            arg: Box::new(t2),
        }
    }

    pub fn reft(t: Term) -> Term {
        Term::Ref(Box::new(t))
    }

    pub fn deref(t: Term) -> Term {
        Term::Deref(Box::new(t))
    }

    pub fn assign(to: Term, body: Term) -> Term {
        Term::Assign {
            to: Box::new(to),
            body: Box::new(body),
        }
    }

    pub fn is_value(&self) -> bool {
        matches!(self, Term::Lambda { .. } | Term::Unit | Term::Loc(_))
    }

    pub fn free_vars(&self) -> HashSet<Var> {
        match self {
            Term::Var(v) => HashSet::from([v.clone()]),
            Term::Const(_) => HashSet::new(),
            Term::Lambda {
                var,
                annot: _,
                body,
            } => {
                let mut vars = body.free_vars();
                vars.remove(var);
                vars
            }
            Term::App { fun, arg } => {
                let mut vars = fun.free_vars();
                vars.extend(arg.free_vars());
                vars
            }
            Term::Assign { to, body } => {
                let mut vars = to.free_vars();
                vars.extend(body.free_vars());
                vars
            }
            Term::Ref(t) => t.free_vars(),
            Term::Deref(t) => t.free_vars(),
            Term::Unit => HashSet::new(),
            Term::Loc(_) => HashSet::new(),
            Term::Let {
                var,
                bound_term,
                in_term,
            } => {
                let mut vars = in_term.free_vars();
                vars.remove(var);
                vars.extend(bound_term.free_vars());
                vars
            }
        }
    }

    pub fn subst(self, v: &Var, t: Term) -> Term {
        match self {
            Term::Var(v1) => {
                if *v == v1 {
                    t
                } else {
                    Term::Var(v1)
                }
            }
            Term::Const(i) => Term::Const(i),
            Term::Lambda { var, annot, body } => {
                if var == *v {
                    Term::Lambda { var, annot, body }
                } else {
                    Term::Lambda {
                        var,
                        annot,
                        body: Box::new((*body).subst(v, t)),
                    }
                }
            }
            Term::App { fun, arg } => Term::App {
                fun: Box::new((*fun).subst(v, t.clone())),
                arg: Box::new((*arg).subst(v, t)),
            },
            Term::Unit => Term::Unit,
            Term::Ref(tm) => Term::Ref(Box::new((*tm).subst(v, t))),
            Term::Deref(tm) => Term::Deref(Box::new((*tm).subst(v, t))),
            Term::Assign { to, body } => Term::Assign {
                to: Box::new((*to).subst(v, t.clone())),
                body: Box::new((*body).subst(v, t)),
            },
            Term::Loc(loc) => Term::Loc(loc),
            Term::Let {
                var,
                bound_term,
                in_term,
            } => {
                let in_subst = in_term.subst(v, t.clone());
                if var == *v {
                    Term::Let {
                        var,
                        bound_term,
                        in_term: Box::new(in_subst),
                    }
                } else {
                    Term::Let {
                        var,
                        bound_term: Box::new(bound_term.subst(v, t)),
                        in_term: Box::new(in_subst),
                    }
                }
            }
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => write!(f, "{v}"),
            Term::Const(c) => write!(f, "{c}"),
            Term::Lambda { var, annot, body } => write!(f, "\\{var}:{annot}.({body})"),
            Term::App { fun, arg } => write!(f, "({fun}) ({arg})"),
            Term::Unit => f.write_str("unit"),
            Term::Ref(t) => write!(f, "ref ({t})"),
            Term::Deref(t) => write!(f, "!({t})"),
            Term::Assign { to, body } => write!(f, "({to}) := ({body})"),
            Term::Loc(loc) => write!(f, "{loc}"),
            Term::Let {
                var,
                bound_term,
                in_term,
            } => write!(f, "let ({var} = {bound_term}) in {in_term}"),
        }
    }
}

impl From<Var> for Term {
    fn from(v: Var) -> Term {
        Term::Var(v)
    }
}

impl From<&str> for Term {
    fn from(s: &str) -> Term {
        Term::Var(s.to_owned())
    }
}

impl From<Loc> for Term {
    fn from(loc: Loc) -> Term {
        Term::Loc(loc)
    }
}

#[cfg(test)]
mod term_tests {
    use super::{Term, Type};
    use std::collections::HashSet;

    fn example_term1() -> Term {
        Term::assign(
            Term::reft(Term::Unit),
            Term::lam("x", Type::Unit, Term::app("y".into(), "x".into())),
        )
    }

    fn example_term2() -> Term {
        Term::deref(Term::app(Term::lam("x", Type::Unit, 0.into()), "y".into()))
    }

    #[test]
    fn seq_terms() {
        let result = example_term1().seq(example_term2());
        let expected = Term::App {
            fun: Box::new(Term::Lambda {
                var: "x0".to_owned(),
                annot: Type::Unit,
                body: Box::new(example_term2()),
            }),
            arg: Box::new(example_term1()),
        };
        assert_eq!(result, expected)
    }

    #[test]
    fn is_val_lam() {
        let result = Term::lam("x", Type::Unit, Term::app("x".into(), "y".into())).is_value();
        assert!(result)
    }

    #[test]
    fn is_val_ref() {
        let result = Term::reft(Term::Unit).is_value();
        assert!(!result)
    }

    #[test]
    fn free_vars1() {
        let result = example_term1().free_vars();
        let expected = HashSet::from(["y".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn free_vars2() {
        let result = example_term2().free_vars();
        let expected = HashSet::from(["y".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst1() {
        let result = example_term1()
            .subst(&"x".to_owned(), Term::Unit)
            .subst(&"y".to_owned(), Term::reft(Term::Unit));
        let expected = Term::assign(
            Term::reft(Term::Unit),
            Term::lam(
                "x",
                Type::Unit,
                Term::app(Term::reft(Term::Unit), "x".into()),
            ),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn subst2() {
        let result = example_term2()
            .subst(&"x".to_owned(), Term::Unit)
            .subst(&"y".to_owned(), Term::reft(Term::Unit));
        let expected = Term::deref(Term::app(
            Term::lam("x", Type::Unit, 0.into()),
            Term::reft(Term::Unit),
        ));
        assert_eq!(result, expected)
    }
}
