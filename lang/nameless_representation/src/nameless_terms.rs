use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Var(usize),
    Lambda(Box<Term>),
    App(Box<Term>, Box<Term>),
}

impl Term {
    pub fn lam(t: Term) -> Term {
        Term::Lambda(Box::new(t))
    }

    pub fn app(t1: Term, t2: Term) -> Term {
        Term::App(Box::new(t1), Box::new(t2))
    }

    pub fn is_value(&self) -> bool {
        matches!(self, Term::Lambda(_) | Term::Var(_))
    }

    pub fn shift(self, d: i64, cutoff: usize) -> Term {
        match self {
            Term::Var(i) => {
                let new_i = if i < cutoff {
                    i
                } else {
                    (d + i as i64) as usize
                };
                Term::Var(new_i)
            }
            Term::Lambda(t) => {
                let new_t = t.shift(d, cutoff + 1);
                Term::lam(new_t)
            }
            Term::App(t1, t2) => {
                let new_t1 = t1.shift(d, cutoff);
                let new_t2 = t2.shift(d, cutoff);
                Term::app(new_t1, new_t2)
            }
        }
    }

    pub fn subst(self, j: usize, t: Term) -> Term {
        match self {
            Term::Var(i) => {
                if i == j {
                    t
                } else {
                    Term::Var(i)
                }
            }
            Term::Lambda(t1) => {
                let t_shifted = t.shift(1, 0);
                let t1_subst = t1.subst(j + 1, t_shifted);
                Term::lam(t1_subst)
            }
            Term::App(t1, t2) => {
                let t1_subst = t1.subst(j, t.clone());
                let t2_subst = t2.subst(j, t);
                Term::app(t1_subst, t2_subst)
            }
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => write!(f, "{v}"),
            Term::Lambda(t) => write!(f, "\\_.{t}"),
            Term::App(t1, t2) => write!(f, "({t1}) ({t2})"),
        }
    }
}

#[cfg(test)]
mod term_tests {
    use super::Term;

    #[test]
    fn shift1() {
        let result = Term::lam(Term::lam(Term::app(
            Term::Var(1),
            Term::app(Term::Var(0), Term::Var(2)),
        )))
        .shift(2, 0);
        let expected = Term::lam(Term::lam(Term::app(
            Term::Var(1),
            Term::app(Term::Var(0), Term::Var(4)),
        )));
        assert_eq!(result, expected)
    }

    #[test]
    fn shift2() {
        let result = Term::lam(Term::app(
            Term::app(Term::Var(0), Term::Var(1)),
            Term::lam(Term::app(
                Term::app(Term::Var(0), Term::Var(1)),
                Term::Var(2),
            )),
        ))
        .shift(2, 0);
        let expected = Term::lam(Term::app(
            Term::app(Term::Var(0), Term::Var(3)),
            Term::lam(Term::app(
                Term::app(Term::Var(0), Term::Var(1)),
                Term::Var(4),
            )),
        ));
        assert_eq!(result, expected)
    }

    #[test]
    fn subst1() {
        let result =
            Term::app(Term::Var(1), Term::lam(Term::lam(Term::Var(3)))).subst(1, Term::Var(0));
        let expected = Term::app(Term::Var(0), Term::lam(Term::lam(Term::Var(2))));
        assert_eq!(result, expected)
    }

    #[test]
    fn subst2() {
        let result = Term::app(Term::Var(1), Term::lam(Term::Var(2)))
            .subst(1, Term::app(Term::Var(0), Term::lam(Term::Var(1))));
        let expected = Term::app(
            Term::app(Term::Var(0), Term::lam(Term::Var(1))),
            Term::lam(Term::app(Term::Var(1), Term::lam(Term::Var(2)))),
        );
        assert_eq!(result, expected)
    }

    #[test]
    fn subst3() {
        let result = Term::lam(Term::app(Term::Var(0), Term::Var(1))).subst(1, Term::Var(0));
        let expected = Term::lam(Term::app(Term::Var(0), Term::Var(1)));
        assert_eq!(result, expected)
    }

    #[test]
    fn subst4() {
        let result = Term::lam(Term::app(Term::Var(2), Term::Var(0))).subst(1, Term::Var(0));
        let expected = Term::lam(Term::app(Term::Var(1), Term::Var(0)));
        assert_eq!(result, expected)
    }
}
