pub mod bool;
pub mod eval;
pub mod parse;
pub mod terms;
pub mod values;

#[cfg(test)]
mod term_tests {
    use super::terms::Term;
    use common::{
        eval::Eval,
        terms::{If, IsZero, Num, Pred, Succ},
        values::Num as NumVal,
    };

    #[test]
    fn eval_simple() {
        let term: Term = Succ::new(Succ::new(Pred::new(Num::new(0)))).into();
        let result = term.eval_start().unwrap();
        let expected = NumVal::new(1).into();
        assert_eq!(result, expected)
    }

    #[test]
    fn eval_complex() {
        let term: Term = If::new(
            IsZero::new(Succ::new(Num::new(0))),
            Pred::new(Succ::new(Num::new(0))),
            Succ::new(Pred::new(Num::new(0))),
        )
        .into();
        let result = term.eval_start().unwrap();
        let expected = NumVal::new(0).into();
        assert_eq!(result, expected)
    }
}
