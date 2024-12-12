use std::{collections::HashSet, fmt};

pub type Var = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Var),
    Lambda(Var, Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl Term {
    pub fn free_vars(&self) -> HashSet<Var> {
        match self {
            Term::Var(v) => HashSet::from([v.clone()]),
            Term::Lambda(v, t) => {
                let mut free_v = t.free_vars();
                free_v.remove(v);
                free_v
            }
            Term::App(t1, t2) => {
                let mut free_v = t1.free_vars();
                free_v.extend(t2.free_vars());
                free_v
            }
        }
    }

    pub fn subst(self, var: &Var, t: Term) -> Term {
        match self {
            Term::Var(v) => {
                if v == *var {
                    t
                } else {
                    Term::Var(v)
                }
            }
            Term::Lambda(v, b) => {
                if v == *var {
                    Term::Lambda(v, b)
                } else {
                    Term::Lambda(v, Box::new(b.subst(var, t)))
                }
            }
            Term::App(t1, t2) => Term::App(
                Box::new(t1.subst(var, t.clone())),
                Box::new(t2.subst(var, t)),
            ),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => write!(f, "{}", v),
            Term::Lambda(v, body) => write!(f, "\\{v}.{body}"),
            Term::App(t1, t2) => write!(f, "({t1} {t2})"),
        }
    }
}

#[cfg(test)]
mod term_tests {
    use super::Term;
    use std::collections::HashSet;

    #[test]
    fn free_v1() {
        let result = Term::Lambda("x".to_owned(), Box::new(Term::Var("x".to_owned()))).free_vars();
        let expected = HashSet::new();
        assert_eq!(result, expected)
    }

    #[test]
    fn free_v2() {
        let result = Term::App(
            Box::new(Term::Lambda(
                "x".to_owned(),
                Box::new(Term::Var("y".to_owned())),
            )),
            Box::new(Term::Var("x".to_owned())),
        )
        .free_vars();
        let expected = HashSet::from(["x".to_owned(), "y".to_owned()]);
        assert_eq!(result, expected)
    }

    #[test]
    fn subst1() {
        let result = Term::Lambda("x".to_owned(), Box::new(Term::Var("x".to_owned())))
            .subst(&"x".to_owned(), Term::Var("y".to_owned()));
        let expected = Term::Lambda("x".to_owned(), Box::new(Term::Var("x".to_owned())));
        assert_eq!(result, expected)
    }

    #[test]
    fn subst2() {
        let result = Term::App(
            Box::new(Term::Lambda(
                "x".to_owned(),
                Box::new(Term::Var("y".to_owned())),
            )),
            Box::new(Term::Var("x".to_owned())),
        )
        .subst(&"x".to_owned(), Term::Var("z".to_owned()))
        .subst(&"y".to_owned(), Term::Var("z".to_owned()));
        let expected = Term::App(
            Box::new(Term::Lambda(
                "x".to_owned(),
                Box::new(Term::Var("z".to_owned())),
            )),
            Box::new(Term::Var("z".to_owned())),
        );
        assert_eq!(result, expected)
    }
}
