use super::{types::Type, values::Value};
use common::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    terms::{
        App, Assign, Deref, False, Fix, If, IsZero, Lambda, Let, Loc, Num, Pred, Ref, Succ, True,
        Unit, Variable,
    },
    TypeVar, Var,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<Term>),
    Num(Num<Term>),
    Succ(Succ<Term>),
    Pred(Pred<Term>),
    IsZero(IsZero<Term>),
    Lambda(Lambda<Term>),
    App(App<Term>),
    Unit(Unit<Term>),
    Ref(Ref<Term>),
    Deref(Deref<Term>),
    Assign(Assign<Term>),
    Loc(Loc<Term>),
    Let(Let<Term>),
    If(If<Term>),
    True(True<Term>),
    False(False<Term>),
    Fix(Fix<Term>),
}

impl common::terms::Term for Term {}

impl LanguageTerm for Term {
    type Type = Type;
    type Value = Value;
}

impl SubstType<Type> for Term {
    type Target = Self;
    fn subst_type(self, _: &TypeVar, _: &Type) -> Self::Target {
        self
    }
}

impl SubstTerm<Term> for Term {
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Num(c) => c.subst(v, t),
            Term::Succ(s) => s.subst(v, t),
            Term::Pred(p) => p.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::Unit(u) => u.subst(v, t),
            Term::Ref(reft) => reft.subst(v, t),
            Term::Deref(dereft) => dereft.subst(v, t),
            Term::Assign(ass) => ass.subst(v, t),
            Term::Loc(loc) => loc.subst(v, t),
            Term::Let(lett) => lett.subst(v, t),
            Term::If(ift) => ift.subst(v, t),
            Term::True(tru) => tru.subst(v, t),
            Term::False(fls) => fls.subst(v, t),
            Term::Fix(fix) => fix.subst(v, t),
            Term::IsZero(isz) => isz.subst(v, t),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => v.fmt(f),
            Term::Num(c) => c.fmt(f),
            Term::Succ(s) => s.fmt(f),
            Term::Pred(p) => p.fmt(f),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::Unit(u) => u.fmt(f),
            Term::Ref(reft) => reft.fmt(f),
            Term::Deref(dereft) => dereft.fmt(f),
            Term::Assign(ass) => ass.fmt(f),
            Term::Loc(loc) => loc.fmt(f),
            Term::Let(lett) => lett.fmt(f),
            Term::If(ift) => ift.fmt(f),
            Term::True(tru) => tru.fmt(f),
            Term::False(fls) => fls.fmt(f),
            Term::Fix(fix) => fix.fmt(f),
            Term::IsZero(isz) => isz.fmt(f),
        }
    }
}

impl From<Variable<Term>> for Term {
    fn from(v: Variable<Term>) -> Term {
        Term::Var(v)
    }
}
impl From<Num<Term>> for Term {
    fn from(n: Num<Term>) -> Term {
        Term::Num(n)
    }
}
impl From<Succ<Term>> for Term {
    fn from(s: Succ<Term>) -> Term {
        Term::Succ(s)
    }
}

impl From<Pred<Term>> for Term {
    fn from(p: Pred<Term>) -> Term {
        Term::Pred(p)
    }
}
impl From<Lambda<Term>> for Term {
    fn from(lam: Lambda<Term>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<App<Term>> for Term {
    fn from(app: App<Term>) -> Term {
        Term::App(app)
    }
}

impl From<Unit<Term>> for Term {
    fn from(u: Unit<Term>) -> Term {
        Term::Unit(u)
    }
}

impl From<Ref<Term>> for Term {
    fn from(reft: Ref<Term>) -> Term {
        Term::Ref(reft)
    }
}

impl From<Deref<Term>> for Term {
    fn from(dereft: Deref<Term>) -> Term {
        Term::Deref(dereft)
    }
}

impl From<Assign<Term>> for Term {
    fn from(ass: Assign<Term>) -> Term {
        Term::Assign(ass)
    }
}

impl From<Loc<Term>> for Term {
    fn from(loc: Loc<Term>) -> Term {
        Term::Loc(loc)
    }
}

impl From<If<Term>> for Term {
    fn from(ift: If<Term>) -> Term {
        Term::If(ift)
    }
}

impl From<True<Term>> for Term {
    fn from(tru: True<Term>) -> Term {
        Term::True(tru)
    }
}

impl From<False<Term>> for Term {
    fn from(fls: False<Term>) -> Term {
        Term::False(fls)
    }
}

impl From<Let<Term>> for Term {
    fn from(lt: Let<Term>) -> Term {
        Term::Let(lt)
    }
}
impl From<Fix<Term>> for Term {
    fn from(fix: Fix<Term>) -> Term {
        Term::Fix(fix)
    }
}

impl From<IsZero<Term>> for Term {
    fn from(isz: IsZero<Term>) -> Term {
        Term::IsZero(isz)
    }
}

#[cfg(test)]
mod term_tests {
    use super::Term;
    use common::{
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
