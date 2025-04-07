use std::fmt;

pub mod bool;
pub mod inductive_definitions;
pub mod parse;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Term {
    True,
    False,
    If(Box<Term>, Box<Term>, Box<Term>),
    Zero,
    Succ(Box<Term>),
    Pred(Box<Term>),
    IsZero(Box<Term>),
}

impl Term {
    pub fn is_value(&self) -> bool {
        self.is_numeric_value() || matches!(self, Term::True | Term::False)
    }

    pub fn is_numeric_value(&self) -> bool {
        match self {
            Term::Zero => true,
            Term::Succ(t) => t.is_numeric_value(),
            _ => false,
        }
    }

    pub fn is_stuck(&self) -> bool {
        self.clone().eval_once().is_none()
    }

    pub fn eval_once(self) -> Option<Term> {
        match self {
            Term::If(t1, t2, t3) => match *t1 {
                Term::True => Some(*t2),
                Term::False => Some(*t3),
                _ => {
                    let t1_evaled = t1.eval_once()?;
                    Some(Term::If(Box::new(t1_evaled), t2, t3))
                }
            },
            Term::Succ(t) => {
                if t.is_numeric_value() {
                    Some(Term::Succ(t))
                } else {
                    let t_evaled = t.eval_once()?;
                    Some(Term::Succ(Box::new(t_evaled)))
                }
            }
            Term::Pred(t) => match *t {
                Term::Zero => Some(Term::Zero),
                Term::Succ(t1) => {
                    if t1.is_numeric_value() {
                        Some(*t1)
                    } else {
                        let t_evaled = Term::Succ(t1).eval_once()?;
                        Some(Term::Pred(Box::new(t_evaled)))
                    }
                }
                _ => {
                    let t_evaled = t.eval_once()?;
                    Some(Term::Pred(Box::new(t_evaled)))
                }
            },
            Term::IsZero(t) => match *t {
                Term::Zero => Some(Term::True),
                Term::Succ(t) => {
                    if t.is_numeric_value() {
                        Some(Term::False)
                    } else {
                        let t_evaled = Term::Succ(t).eval_once()?;
                        Some(Term::IsZero(Box::new(t_evaled)))
                    }
                }
                _ => {
                    let t_evaled = t.eval_once()?;
                    Some(Term::IsZero(Box::new(t_evaled)))
                }
            },
            _ => {
                if self.is_value() {
                    Some(self)
                } else {
                    None
                }
            }
        }
    }

    pub fn eval(self) -> Term {
        match self.clone().eval_once() {
            None => self,
            Some(t) => {
                if t.is_value() {
                    t
                } else {
                    t.eval()
                }
            }
        }
    }

    pub fn eval_big(self) -> Option<Term> {
        match self {
            Term::If(t1, t2, t3) => match t1.eval_big()? {
                Term::True => t2.eval_big(),
                Term::False => t3.eval_big(),
                _ => None,
            },
            Term::Succ(t) => {
                let t_evaled = t.eval_big()?;
                if t_evaled.is_numeric_value() {
                    Some(Term::Succ(Box::new(t_evaled)))
                } else {
                    None
                }
            }
            Term::Pred(t) => match t.eval_big()? {
                Term::Zero => Some(Term::Zero),
                Term::Succ(t) => {
                    if t.is_numeric_value() {
                        Some(*t)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            Term::IsZero(t) => {
                let t_evaled = t.eval_big()?;
                if let Term::Zero = t_evaled {
                    Some(Term::True)
                } else if t_evaled.is_numeric_value() {
                    Some(Term::False)
                } else {
                    None
                }
            }
            _ => Some(self),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::True => f.write_str("true"),
            Term::False => f.write_str("false"),
            Term::If(ift, thent, elset) => write!(f, "if ({ift}) {{ {thent} }} else {{ {elset} }}"),
            Term::Zero => f.write_str("zero"),
            Term::Succ(t) => write!(f, "succ({t})"),
            Term::Pred(t) => write!(f, "pred({t})"),
            Term::IsZero(t) => write!(f, "iszero({t})"),
        }
    }
}

#[cfg(test)]
mod term_tests {
    use super::Term;

    #[test]
    fn is_val_true() {
        let result = Term::True.is_value();
        assert!(result)
    }

    #[test]
    fn is_val_false() {
        let result = Term::Succ(Box::new(Term::True)).is_value();
        assert!(!result)
    }

    #[test]
    fn eval_simple() {
        let result = Term::Succ(Box::new(Term::Succ(Box::new(Term::Pred(Box::new(
            Term::Zero,
        ))))))
        .eval();
        let expected = Term::Succ(Box::new(Term::Succ(Box::new(Term::Zero))));
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_complex() {
        let result = Term::If(
            Box::new(Term::IsZero(Box::new(Term::Succ(Box::new(Term::Zero))))),
            Box::new(Term::Pred(Box::new(Term::Succ(Box::new(Term::Zero))))),
            Box::new(Term::Succ(Box::new(Term::Pred(Box::new(Term::Zero))))),
        )
        .eval();
        let expected = Term::Succ(Box::new(Term::Zero));
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_big() {
        let result = Term::If(
            Box::new(Term::IsZero(Box::new(Term::Succ(Box::new(Term::Zero))))),
            Box::new(Term::Pred(Box::new(Term::Succ(Box::new(Term::Zero))))),
            Box::new(Term::Succ(Box::new(Term::Pred(Box::new(Term::Zero))))),
        )
        .eval_big()
        .unwrap();
        let expected = Term::Succ(Box::new(Term::Zero));
        assert_eq!(result, expected)
    }
}
