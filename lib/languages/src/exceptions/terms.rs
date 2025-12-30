use super::Exceptions;
use macros::{
    Eval, FreeTypeVars, FreeVars, FromVariants, GenerateConstraintsTerm, GrammarDescribe,
    LangDisplay, LatexFmt, Spanned, SubstTerm, SubstType, Typecheck,
};
use syntax::terms::{
    App, Exception, False, If, IsZero, Lambda, Num, Pred, Raise, Succ, True, Try, TryWithVal, Unit,
    Variable,
};

#[derive(
    GenerateConstraintsTerm,
    FreeVars,
    FreeTypeVars,
    Spanned,
    FromVariants,
    SubstType,
    SubstTerm,
    LatexFmt,
    LangDisplay,
    GrammarDescribe,
    Eval,
    Typecheck,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[Lang(Exceptions)]
pub enum Term {
    Variable(Variable<Exceptions>),
    Num(Num<Exceptions>),
    True(True<Exceptions>),
    False(False<Exceptions>),
    Succ(Succ<Exceptions>),
    Pred(Pred<Exceptions>),
    IsZero(IsZero<Exceptions>),
    If(If<Exceptions>),
    Lambda(Lambda<Exceptions>),
    App(App<Exceptions>),
    Unit(Unit<Exceptions>),
    Exception(Exception<Exceptions>),
    Try(Try<Exceptions>),
    Raise(Raise<Exceptions>),
    TryWithVal(TryWithVal<Exceptions>),
}

impl syntax::terms::Term for Term {}

#[cfg(test)]
pub mod term_tests {
    use super::{App, Exceptions, Lambda, Raise, Term, Try, TryWithVal, Unit, Variable};
    use eval::Eval;
    use syntax::{span::Span, types::Unit as UnitTy, values::Unit as UnitVal};

    pub fn example_term1() -> Term {
        Try::<Exceptions>::new(
            App::<Exceptions>::new(
                Lambda::<Exceptions>::new(
                    "x",
                    UnitTy::new(Span::default()),
                    Variable::<Exceptions>::new("x", Span::default()),
                    Span::default(),
                ),
                Unit::<Exceptions>::new(Span::default()),
            ),
            Unit::<Exceptions>::new(Span::default()),
            Span::default(),
        )
        .into()
    }

    pub fn example_term2() -> Term {
        TryWithVal::<Exceptions>::new(
            Raise::<Exceptions>::new(
                Unit::<Exceptions>::new(Span::default()),
                UnitTy::new(Span::default()),
                UnitTy::new(Span::default()),
                Span::default(),
            ),
            Lambda::<Exceptions>::new(
                "x",
                UnitTy::new(Span::default()),
                Unit::new(Span::default()),
                Span::default(),
            ),
            Span::default(),
        )
        .into()
    }

    #[test]
    fn eval1() {
        let result = example_term1().eval_start().unwrap();
        let expected = UnitVal::new(Span::default()).into();
        assert_eq!(result.val(), expected)
    }

    #[test]
    fn eval2() {
        let result = example_term2().eval_start().unwrap();
        let expected = UnitVal::new(Span::default()).into();
        assert_eq!(result.val(), expected)
    }
}
