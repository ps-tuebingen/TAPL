use super::{ClassName, FieldName, MethodName, Var};
use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Var),
    Const(i64),
    FieldProjection(Box<Term>, FieldName),
    MethodCall(Box<Term>, MethodName, Vec<Term>),
    New(ClassName, Vec<Term>),
    Cast(ClassName, Box<Term>),
}

impl Term {
    pub fn is_value(&self) -> bool {
        if let Term::New(_, args) = self {
            args.iter().all(|next| next.is_value())
        } else {
            false
        }
    }

    pub fn subst(self, subst: &HashMap<Var, Term>) -> Term {
        match self {
            Term::Var(v) => subst.get(&v).cloned().unwrap_or(Term::Var(v)),
            Term::Const(i) => Term::Const(i),
            Term::FieldProjection(t, field) => {
                Term::FieldProjection(Box::new((*t).subst(subst)), field)
            }
            Term::MethodCall(t, name, args) => Term::MethodCall(
                Box::new((*t).subst(subst)),
                name,
                args.into_iter().map(|arg| arg.subst(subst)).collect(),
            ),
            Term::New(class, args) => Term::New(
                class,
                args.into_iter().map(|arg| arg.subst(subst)).collect(),
            ),
            Term::Cast(class, t) => Term::Cast(class, Box::new((*t).subst(subst))),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => f.write_str(v),
            Term::Const(i) => write!(f, "{i}"),
            Term::FieldProjection(t, fl) => write!(f, "{t}.{fl}"),
            Term::MethodCall(t, m, args) => write!(
                f,
                "{t}.{m}({})",
                args.iter()
                    .map(|arg| format!("{arg}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Term::New(cl, args) => write!(
                f,
                "new {cl}({})",
                args.iter()
                    .map(|arg| format!("{arg}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Term::Cast(c, t) => write!(f, "({c}) {t}"),
        }
    }
}

#[cfg(test)]
mod term_tests {
    use super::Term;
    use std::collections::HashMap;

    #[test]
    fn new_value() {
        let term = Term::New(
            "Pair".to_owned(),
            vec![
                Term::New("A".to_owned(), vec![]),
                Term::New("B".to_owned(), vec![]),
            ],
        );
        assert!(term.is_value())
    }

    #[test]
    fn new_nonvalue() {
        let term = Term::New(
            "Pair".to_owned(),
            vec![
                Term::Cast(
                    "Object".to_owned(),
                    Box::new(Term::New("A".to_owned(), vec![])),
                ),
                Term::New("B".to_owned(), vec![]),
            ],
        );
        assert!(!term.is_value())
    }

    #[test]
    fn subst_term() {
        let result = Term::New(
            "Pair".to_owned(),
            vec![Term::Var("a".to_owned()), Term::Var("b".to_owned())],
        )
        .subst(&HashMap::from([
            ("a".to_owned(), Term::New("A".to_owned(), vec![])),
            ("b".to_owned(), Term::New("B".to_owned(), vec![])),
        ]));
        let expected = Term::New(
            "Pair".to_owned(),
            vec![
                Term::New("A".to_owned(), vec![]),
                Term::New("B".to_owned(), vec![]),
            ],
        );
        assert_eq!(result, expected)
    }
}
