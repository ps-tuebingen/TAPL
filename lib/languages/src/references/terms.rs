use super::{References, types::Type};
use latex::{LatexConfig, LatexFmt};
use macros::{Eval, GrammarDescribe, TermDisplay, Typecheck};
use std::fmt;
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{
        App, Assign, Deref, False, Fix, If, IsZero, Lambda, Let, Loc, Num, Pred, Ref, Succ, True,
        Unit, Variable,
    },
};

#[derive(TermDisplay, GrammarDescribe, Eval, Typecheck, Debug, Clone, PartialEq, Eq)]
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

impl SubstType for Term {
    type Lang = References;
    type Target = Self;
    fn subst_type(self, _: &TypeVar, _: &Type) -> Self::Target {
        self
    }
}

impl SubstTerm for Term {
    type Lang = References;
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Variable(var) => var.subst(v, t),
            Term::Num(c) => c.subst(v, t).into(),
            Term::Succ(s) => s.subst(v, t).into(),
            Term::Pred(p) => p.subst(v, t).into(),
            Term::Lambda(lam) => lam.subst(v, t).into(),
            Term::App(app) => app.subst(v, t).into(),
            Term::Unit(u) => u.subst(v, t).into(),
            Term::Ref(reft) => reft.subst(v, t).into(),
            Term::Deref(dereft) => dereft.subst(v, t).into(),
            Term::Assign(ass) => ass.subst(v, t).into(),
            Term::Loc(loc) => loc.subst(v, t).into(),
            Term::Let(lett) => lett.subst(v, t).into(),
            Term::If(ift) => ift.subst(v, t).into(),
            Term::True(tru) => tru.subst(v, t).into(),
            Term::False(fls) => fls.subst(v, t).into(),
            Term::Fix(fix) => fix.subst(v, t).into(),
            Term::IsZero(isz) => isz.subst(v, t).into(),
        }
    }
}

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Variable(v) => v.to_latex(conf),
            Term::Num(c) => c.to_latex(conf),
            Term::Succ(s) => s.to_latex(conf),
            Term::Pred(p) => p.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::Unit(u) => u.to_latex(conf),
            Term::Ref(reft) => reft.to_latex(conf),
            Term::Deref(dereft) => dereft.to_latex(conf),
            Term::Assign(ass) => ass.to_latex(conf),
            Term::Loc(loc) => loc.to_latex(conf),
            Term::Let(lett) => lett.to_latex(conf),
            Term::If(ift) => ift.to_latex(conf),
            Term::True(tru) => tru.to_latex(conf),
            Term::False(fls) => fls.to_latex(conf),
            Term::Fix(fix) => fix.to_latex(conf),
            Term::IsZero(isz) => isz.to_latex(conf),
        }
    }
}

impl From<Variable<References>> for Term {
    fn from(v: Variable<References>) -> Term {
        Term::Variable(v)
    }
}
impl From<Num<References>> for Term {
    fn from(n: Num<References>) -> Term {
        Term::Num(n)
    }
}
impl From<Succ<References>> for Term {
    fn from(s: Succ<References>) -> Term {
        Term::Succ(s)
    }
}

impl From<Pred<References>> for Term {
    fn from(p: Pred<References>) -> Term {
        Term::Pred(p)
    }
}
impl From<Lambda<References>> for Term {
    fn from(lam: Lambda<References>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<App<References>> for Term {
    fn from(app: App<References>) -> Term {
        Term::App(app)
    }
}

impl From<Unit<References>> for Term {
    fn from(u: Unit<References>) -> Term {
        Term::Unit(u)
    }
}

impl From<Ref<References>> for Term {
    fn from(reft: Ref<References>) -> Term {
        Term::Ref(reft)
    }
}

impl From<Deref<References>> for Term {
    fn from(dereft: Deref<References>) -> Term {
        Term::Deref(dereft)
    }
}

impl From<Assign<References>> for Term {
    fn from(ass: Assign<References>) -> Term {
        Term::Assign(ass)
    }
}

impl From<Loc<References>> for Term {
    fn from(loc: Loc<References>) -> Term {
        Term::Loc(loc)
    }
}

impl From<If<References>> for Term {
    fn from(ift: If<References>) -> Term {
        Term::If(ift)
    }
}

impl From<True<References>> for Term {
    fn from(tru: True<References>) -> Term {
        Term::True(tru)
    }
}

impl From<False<References>> for Term {
    fn from(fls: False<References>) -> Term {
        Term::False(fls)
    }
}

impl From<Let<References>> for Term {
    fn from(lt: Let<References>) -> Term {
        Term::Let(lt)
    }
}
impl From<Fix<References>> for Term {
    fn from(fix: Fix<References>) -> Term {
        Term::Fix(fix)
    }
}

impl From<IsZero<References>> for Term {
    fn from(isz: IsZero<References>) -> Term {
        Term::IsZero(isz)
    }
}

#[cfg(test)]
mod term_tests {
    use super::Term;
    use syntax::{
        subst::SubstTerm,
        terms::{App, Assign, Deref, Lambda, Num, Ref, Unit, Variable},
        types::Unit as UnitTy,
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
}

#[cfg(test)]
mod check_tests {
    use super::{References, Term};
    use eval::Eval;
    use syntax::{
        terms::{App, Assign, Deref, Lambda, Loc, Ref, Unit, Variable},
        types::{Reference, Unit as UnitTy},
        values::Unit as UnitVal,
    };

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
