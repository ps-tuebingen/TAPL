use super::References;
use macros::{
    Eval, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType, Typecheck,
};
use syntax::terms::{
    App, Assign, Deref, False, Fix, If, IsZero, Lambda, Let, Loc, Num, Pred, Ref, Succ, True, Unit,
    Variable,
};

#[derive(
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
        subst::SubstTerm,
        terms::{App, Assign, Deref, Lambda, Loc, Num, Ref, Unit, Variable},
        types::{Reference, Unit as UnitTy},
        values::Unit as UnitVal,
    };

    fn example_term1() -> Term {
        Assign::new(
            Ref::new(Unit::new()),
            Lambda::new(
                "x",
                UnitTy::new(),
                App::new(Variable::new("y"), Variable::new("x")),
            ),
        )
        .into()
    }

    fn example_term2() -> Term {
        Deref::new(App::new(
            Lambda::new("x", UnitTy::new(), Num::new(0)),
            Variable::new("y"),
        ))
        .into()
    }

    #[test]
    fn subst1() {
        let result = example_term1()
            .subst(&"x".to_owned(), &Unit::new().into())
            .subst(&"y".to_owned(), &Ref::new(Unit::new()).into());
        let expected = Assign::new(
            Ref::new(Unit::new()),
            Lambda::new(
                "x",
                UnitTy::new(),
                App::new(Ref::new(Unit::new()), Variable::new("x")),
            ),
        )
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn subst2() {
        let result = example_term2()
            .subst(&"x".to_owned(), &Unit::new().into())
            .subst(&"y".to_owned(), &Ref::new(Unit::new()).into());
        let expected = Deref::new(App::new(
            Lambda::new("x", UnitTy::new(), Num::new(0)),
            Ref::new(Unit::new()),
        ))
        .into();
        assert_eq!(result, expected)
    }

    #[test]
    fn eval1() {
        let term: Term = App::new(
            Lambda::new(
                "x",
                Reference::new(UnitTy::new()),
                Deref::new(Variable::new("x")),
            ),
            App::new(
                Lambda::new("y", UnitTy::new(), Ref::new(Variable::new("y"))),
                Unit::new(),
            ),
        )
        .into();
        let result = term.eval(&mut Default::default()).unwrap();
        let expected = UnitVal::new().into();
        assert_eq!(result.val(), expected)
    }

    #[test]
    fn eval2() {
        let term: Term = App::new(
            Lambda::new(
                "x",
                Reference::new(UnitTy::new()),
                Assign::new(Variable::new("x"), Deref::new(Variable::new("x"))),
            ),
            Ref::new(Unit::new()),
        )
        .into();
        let result = term.eval(&mut Default::default()).unwrap();
        let expected = UnitVal::new().into();
        assert_eq!(result.val(), expected)
    }

    #[test]
    fn eval_store() {
        let term: Term = App::<References>::seq(
            Assign::new(
                Ref::new(Unit::new()),
                App::new(
                    Lambda::new("x", UnitTy::new(), Variable::new("x")),
                    Unit::new(),
                ),
            ),
            Deref::new(Loc::new(0)),
        )
        .into();
        let result = term.eval(&mut Default::default()).unwrap();
        let expected = UnitVal::new().into();
        assert_eq!(result.val(), expected)
    }
}
