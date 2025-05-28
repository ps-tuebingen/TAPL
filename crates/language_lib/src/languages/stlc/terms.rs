use super::{types::Type, values::Value};
use std::fmt;
use syntax::{
    subst::{SubstTerm, SubstType},
    terms::{
        App, Ascribe, Cons, False, Fix, Fst, Head, If, IsNil, IsZero, Lambda, Left, Let, Nil,
        Nothing, Num, Pair, Pred, Projection, Record, RecordProj, Right, Snd, SomeCase, Something,
        Succ, SumCase, Tail, True, Tuple, Unit, Variable, Variant, VariantCase,
    },
    TypeVar, Var,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<Term>),
    Lambda(Lambda<Term>),
    App(App<Term>),
    Unit(Unit<Term>),
    True(True<Term>),
    False(False<Term>),
    If(If<Term>),
    Num(Num<Term>),
    Pred(Pred<Term>),
    Succ(Succ<Term>),
    IsZero(IsZero<Term>),
    Ascribe(Ascribe<Term>),
    Let(Let<Term>),
    Pair(Pair<Term>),
    Fst(Fst<Term>),
    Snd(Snd<Term>),
    Tuple(Tuple<Term>),
    Projection(Projection<Term>),
    Record(Record<Term>),
    RecordProj(RecordProj<Term>),
    Left(Left<Term>),
    Right(Right<Term>),
    SumCase(SumCase<Term>),
    Variant(Variant<Term>),
    VariantCase(VariantCase<Term>),
    Nothing(Nothing<Term>),
    Something(Something<Term>),
    SomeCase(SomeCase<Term>),
    Fix(Fix<Term>),
    Nil(Nil<Term>),
    Cons(Cons<Term>),
    IsNil(IsNil<Term>),
    Head(Head<Term>),
    Tail(Tail<Term>),
}

impl common::terms::Term for Term {}

impl LanguageTerm for Term {
    type Type = Type;
    type Value = Value;
}

impl SubstType<Type> for Term {
    type Target = Term;
    fn subst_type(self, _: &TypeVar, _: &Type) -> Self::Target {
        self
    }
}

impl SubstTerm<Term> for Term {
    type Target = Term;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::True(tru) => tru.subst(v, t),
            Term::False(fls) => fls.subst(v, t),
            Term::Unit(u) => u.subst(v, t),
            Term::If(ift) => ift.subst(v, t),
            Term::Num(num) => num.subst(v, t),
            Term::Pred(p) => p.subst(v, t),
            Term::Succ(s) => s.subst(v, t),
            Term::IsZero(isz) => isz.subst(v, t),
            Term::Ascribe(asc) => asc.subst(v, t),
            Term::Let(lt) => lt.subst(v, t),
            Term::Pair(pr) => pr.subst(v, t),
            Term::Tuple(tup) => tup.subst(v, t),
            Term::Projection(proj) => proj.subst(v, t),
            Term::Fst(proj) => proj.subst(v, t),
            Term::Snd(proj) => proj.subst(v, t),
            Term::Record(rec) => rec.subst(v, t),
            Term::RecordProj(proj) => proj.subst(v, t),
            Term::Left(lf) => lf.subst(v, t),
            Term::Right(rt) => rt.subst(v, t),
            Term::SumCase(case) => case.subst(v, t),
            Term::Variant(var) => var.subst(v, t),
            Term::VariantCase(case) => case.subst(v, t),
            Term::Nothing(not) => not.subst(v, t),
            Term::Something(some) => some.subst(v, t),
            Term::SomeCase(case) => case.subst(v, t),
            Term::Fix(fix) => fix.subst(v, t),
            Term::Nil(nil) => nil.subst(v, t),
            Term::Cons(cons) => cons.subst(v, t),
            Term::IsNil(isnil) => isnil.subst(v, t),
            Term::Head(hd) => hd.subst(v, t),
            Term::Tail(tl) => tl.subst(v, t),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => v.fmt(f),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::Unit(unit) => unit.fmt(f),
            Term::True(tru) => tru.fmt(f),
            Term::False(fls) => fls.fmt(f),
            Term::If(ift) => ift.fmt(f),
            Term::Num(num) => num.fmt(f),
            Term::Pred(p) => p.fmt(f),
            Term::Succ(s) => s.fmt(f),
            Term::IsZero(isz) => isz.fmt(f),
            Term::Ascribe(asc) => asc.fmt(f),
            Term::Let(lt) => lt.fmt(f),
            Term::Pair(pr) => pr.fmt(f),
            Term::Tuple(tup) => tup.fmt(f),
            Term::Projection(proj) => proj.fmt(f),
            Term::Fst(proj) => proj.fmt(f),
            Term::Snd(proj) => proj.fmt(f),
            Term::Record(rec) => rec.fmt(f),
            Term::RecordProj(proj) => proj.fmt(f),
            Term::Left(lf) => lf.fmt(f),
            Term::Right(rt) => rt.fmt(f),
            Term::SumCase(case) => case.fmt(f),
            Term::Variant(var) => var.fmt(f),
            Term::VariantCase(case) => case.fmt(f),
            Term::Nothing(not) => not.fmt(f),
            Term::Something(some) => some.fmt(f),
            Term::SomeCase(case) => case.fmt(f),
            Term::Fix(fix) => fix.fmt(f),
            Term::Nil(nil) => nil.fmt(f),
            Term::Cons(cons) => cons.fmt(f),
            Term::IsNil(isnil) => isnil.fmt(f),
            Term::Head(hd) => hd.fmt(f),
            Term::Tail(tl) => tl.fmt(f),
        }
    }
}

impl From<Lambda<Term>> for Term {
    fn from(lam: Lambda<Term>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<Unit<Term>> for Term {
    fn from(u: Unit<Term>) -> Term {
        Term::Unit(u)
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

impl From<Num<Term>> for Term {
    fn from(num: Num<Term>) -> Term {
        Term::Num(num)
    }
}

impl From<Pair<Term>> for Term {
    fn from(pair: Pair<Term>) -> Term {
        Term::Pair(pair)
    }
}

impl From<Tuple<Term>> for Term {
    fn from(tup: Tuple<Term>) -> Term {
        Term::Tuple(tup)
    }
}

impl From<Record<Term>> for Term {
    fn from(rec: Record<Term>) -> Term {
        Term::Record(rec)
    }
}

impl From<Left<Term>> for Term {
    fn from(lft: Left<Term>) -> Term {
        Term::Left(lft)
    }
}

impl From<Right<Term>> for Term {
    fn from(right: Right<Term>) -> Term {
        Term::Right(right)
    }
}

impl From<Variant<Term>> for Term {
    fn from(var: Variant<Term>) -> Term {
        Term::Variant(var)
    }
}

impl From<Nothing<Term>> for Term {
    fn from(not: Nothing<Term>) -> Term {
        Term::Nothing(not)
    }
}

impl From<Something<Term>> for Term {
    fn from(some: Something<Term>) -> Term {
        Term::Something(some)
    }
}

impl From<Nil<Term>> for Term {
    fn from(nil: Nil<Term>) -> Term {
        Term::Nil(nil)
    }
}

impl From<Cons<Term>> for Term {
    fn from(cons: Cons<Term>) -> Term {
        Term::Cons(cons)
    }
}

impl From<Variable<Term>> for Term {
    fn from(var: Variable<Term>) -> Term {
        Term::Var(var)
    }
}

impl From<App<Term>> for Term {
    fn from(app: App<Term>) -> Term {
        Term::App(app)
    }
}

impl From<If<Term>> for Term {
    fn from(ift: If<Term>) -> Term {
        Term::If(ift)
    }
}

impl From<Pred<Term>> for Term {
    fn from(pred: Pred<Term>) -> Term {
        Term::Pred(pred)
    }
}

impl From<Succ<Term>> for Term {
    fn from(succ: Succ<Term>) -> Term {
        Term::Succ(succ)
    }
}

impl From<IsZero<Term>> for Term {
    fn from(isz: IsZero<Term>) -> Term {
        Term::IsZero(isz)
    }
}

impl From<Ascribe<Term>> for Term {
    fn from(asc: Ascribe<Term>) -> Term {
        Term::Ascribe(asc)
    }
}

impl From<Let<Term>> for Term {
    fn from(lt: Let<Term>) -> Term {
        Term::Let(lt)
    }
}

impl From<Projection<Term>> for Term {
    fn from(proj: Projection<Term>) -> Term {
        Term::Projection(proj)
    }
}

impl From<Fst<Term>> for Term {
    fn from(fst: Fst<Term>) -> Term {
        Term::Fst(fst)
    }
}

impl From<Snd<Term>> for Term {
    fn from(snd: Snd<Term>) -> Term {
        Term::Snd(snd)
    }
}

impl From<RecordProj<Term>> for Term {
    fn from(proj: RecordProj<Term>) -> Term {
        Term::RecordProj(proj)
    }
}

impl From<SumCase<Term>> for Term {
    fn from(case: SumCase<Term>) -> Term {
        Term::SumCase(case)
    }
}

impl From<VariantCase<Term>> for Term {
    fn from(case: VariantCase<Term>) -> Term {
        Term::VariantCase(case)
    }
}

impl From<SomeCase<Term>> for Term {
    fn from(case: SomeCase<Term>) -> Term {
        Term::SomeCase(case)
    }
}

impl From<Fix<Term>> for Term {
    fn from(fix: Fix<Term>) -> Term {
        Term::Fix(fix)
    }
}

impl From<IsNil<Term>> for Term {
    fn from(isn: IsNil<Term>) -> Term {
        Term::IsNil(isn)
    }
}

impl From<Head<Term>> for Term {
    fn from(hd: Head<Term>) -> Term {
        Term::Head(hd)
    }
}

impl From<Tail<Term>> for Term {
    fn from(tl: Tail<Term>) -> Term {
        Term::Tail(tl)
    }
}
