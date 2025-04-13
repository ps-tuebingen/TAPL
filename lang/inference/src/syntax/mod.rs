use common::Var;
use std::fmt;

pub mod ascription;
pub mod bool;
pub mod fix;
pub mod lambda;
pub mod let_exp;
pub mod list;
pub mod nat;
pub mod optional;
pub mod pair;
pub mod sum;
pub mod unit;

pub use ascription::Ascribe;
pub use bool::{False, If, True};
pub use fix::Fix;
pub use lambda::{App, Lambda};
pub use let_exp::Let;
pub use list::{Cons, Head, IsNil, Nil, Tail};
pub use nat::{IsZero, Pred, Succ, Zero};
pub use optional::{Nothing, SomeCase, Something};
pub use pair::{Pair, Proj1, Proj2};
pub use sum::{Left, Right, SumCase};
pub use unit::Unit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Var),
    Lambda(Lambda),
    App(App),
    Unit(Unit),
    True(True),
    False(False),
    If(If),
    Zero(Zero),
    Pred(Pred),
    Succ(Succ),
    IsZero(IsZero),
    Ascribe(Ascribe),
    Let(Let),
    Pair(Pair),
    Proj1(Proj1),
    Proj2(Proj2),
    Left(Left),
    Right(Right),
    SumCase(SumCase),
    Nothing(Nothing),
    Something(Something),
    SomeCase(SomeCase),
    Fix(Fix),
    Nil(Nil),
    Cons(Cons),
    IsNil(IsNil),
    Head(Head),
    Tail(Tail),
}

impl From<Var> for Term {
    fn from(var: Var) -> Term {
        Term::Var(var)
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
            Term::Zero(z) => z.fmt(f),
            Term::Pred(p) => p.fmt(f),
            Term::Succ(s) => s.fmt(f),
            Term::IsZero(isz) => isz.fmt(f),
            Term::Ascribe(asc) => asc.fmt(f),
            Term::Let(lt) => lt.fmt(f),
            Term::Pair(pr) => pr.fmt(f),
            Term::Proj1(proj) => proj.fmt(f),
            Term::Proj2(proj) => proj.fmt(f),
            Term::Left(lf) => lf.fmt(f),
            Term::Right(rt) => rt.fmt(f),
            Term::SumCase(case) => case.fmt(f),
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
