pub mod bool;
pub mod eval;
pub mod parse;
pub mod terms;
pub mod values;

#[cfg(test)]
mod term_tests {
    use super::{terms::Term, values::Value};
    use common::eval::Eval;

    #[test]
    fn eval_simple() {
        let result = Term::Succ(Box::new(Term::Succ(Box::new(Term::Pred(Box::new(
            Term::Zero,
        ))))))
        .eval(Default::default())
        .unwrap();
        let expected = Value::Numerical(1);
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_complex() {
        let result = Term::If(
            Box::new(Term::IsZero(Box::new(Term::Succ(Box::new(Term::Zero))))),
            Box::new(Term::Pred(Box::new(Term::Succ(Box::new(Term::Zero))))),
            Box::new(Term::Succ(Box::new(Term::Pred(Box::new(Term::Zero))))),
        )
        .eval(Default::default())
        .unwrap();
        let expected = Value::Numerical(0);
        assert_eq!(result, expected)
    }
}
