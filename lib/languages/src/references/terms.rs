use super::References;
use macros::{
    Eval, FreeTypeVars, FreeVars, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, Spanned,
    SubstTerm, SubstType, Typecheck,
};
use syntax::terms::{
    App, Assign, Deref, False, Fix, If, IsZero, Lambda, Let, Loc, Num, Pred, Ref, Succ, True, Unit,
    Variable,
};

#[derive(
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
#[Lang(References)]
pub enum Term {
    Variable(Variable<References>),
    Num(Num<References>),
    Succ(Succ<References>),
    Pred(Pred<References>),
    IsZero(IsZero<References>),
    Lambda(Lambda<References>),
    App(App<References>),
    Unit(Unit<References>),
    Ref(Ref<References>),
    Deref(Deref<References>),
    Assign(Assign<References>),
    Loc(Loc<References>),
    Let(Let<References>),
    If(If<References>),
    True(True<References>),
    False(False<References>),
    Fix(Fix<References>),
}

impl syntax::terms::Term for Term {}

#[cfg(test)]
mod term_tests {
    use super::{References, Term};
    use eval::Eval;
    use syntax::{
        span::Span,
        subst::SubstTerm,
        terms::{App, Assign, Deref, Lambda, Loc, Num, Ref, Unit, Variable},
        types::{Reference, Unit as UnitTy},
        values::Unit as UnitVal,
    };

    fn example_term1() -> Term {
        Assign::new(
            Ref::new(Unit::new(Span::default()), Span::default()),
            Lambda::new(
                "x",
                UnitTy::new(Span::default()),
                App::new(
                    Variable::new("y", Span::default()),
                    Variable::new("x", Span::default()),
                ),
                Span::default(),
            ),
        )
        .into()
    }

    fn example_term2() -> Term {
        Deref::new(
            App::new(
                Lambda::new(
                    "x",
                    UnitTy::new(Span::default()),
                    Num::new(0, Span::default()),
                    Span::default(),
                ),
                Variable::new("y", Span::default()),
            ),
            Span::default(),
        )
        .into()
    }

    #[test]
    fn subst1() {
        let result = example_term1()
            .subst(&"x".to_owned(), &Unit::new(Span::default()).into())
            .subst(
                &"y".to_owned(),
                &Ref::new(Unit::new(Span::default()), Span::default()).into(),
            );
        let expected = Assign::new(
            Ref::new(Unit::new(Span::default()), Span::default()),
            Lambda::new(
                "x",
                UnitTy::new(Span::default()),
                App::new(
                    Ref::new(Unit::new(Span::default()), Span::default()),
                    Variable::new("x", Span::default()),
                ),
                Span::default(),
            ),
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst2() {
        let result = example_term2()
            .subst(&"x".to_owned(), &Unit::new(Span::default()).into())
            .subst(
                &"y".to_owned(),
                &Ref::new(Unit::new(Span::default()), Span::default()).into(),
            );
        let expected = Deref::new(
            App::new(
                Lambda::new(
                    "x",
                    UnitTy::new(Span::default()),
                    Num::new(0, Span::default()),
                    Span::default(),
                ),
                Ref::new(Unit::new(Span::default()), Span::default()),
            ),
            Span::default(),
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn eval1() {
        let term: Term = App::new(
            Lambda::new(
                "x",
                Reference::new(UnitTy::new(Span::default()), Span::default()),
                Deref::new(Variable::new("x", Span::default()), Span::default()),
                Span::default(),
            ),
            App::new(
                Lambda::new(
                    "y",
                    UnitTy::new(Span::default()),
                    Ref::new(Variable::new("y", Span::default()), Span::default()),
                    Span::default(),
                ),
                Unit::new(Span::default()),
            ),
        )
        .into();
        let result = term.eval(&mut Default::default()).unwrap();
        let expected = UnitVal::new(Span::default()).into();
        assert_eq!(result.val(), expected)
    }

    #[test]
    fn eval2() {
        let term: Term = App::new(
            Lambda::new(
                "x",
                Reference::new(UnitTy::new(Span::default()), Span::default()),
                Assign::new(
                    Variable::new("x", Span::default()),
                    Deref::new(Variable::new("x", Span::default()), Span::default()),
                ),
                Span::default(),
            ),
            Ref::new(Unit::new(Span::default()), Span::default()),
        )
        .into();
        let result = term.eval(&mut Default::default()).unwrap();
        let expected = UnitVal::new(Span::default()).into();
        assert_eq!(result.val(), expected)
    }

    #[test]
    fn eval_store() {
        let term: Term = App::<References>::seq(
            Assign::new(
                Ref::new(Unit::new(Span::default()), Span::default()),
                App::new(
                    Lambda::new(
                        "x",
                        UnitTy::new(Span::default()),
                        Variable::new("x", Span::default()),
                        Span::default(),
                    ),
                    Unit::new(Span::default()),
                ),
            ),
            Deref::new(Loc::new(0, Span::default()), Span::default()),
        )
        .into();
        let result = term.eval(&mut Default::default()).unwrap();
        let expected = UnitVal::new(Span::default()).into();
        assert_eq!(result.val(), expected)
    }
}
