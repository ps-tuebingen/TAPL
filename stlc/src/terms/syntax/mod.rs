use crate::Var;
use std::fmt;

pub mod ascription;
pub mod bool;
pub mod fix;
pub mod lambda;
pub mod let_exp;
pub mod list;
pub mod optional;
pub mod pair;
pub mod record;
pub mod sum;
pub mod tuple;
pub mod unit;
pub mod variant;

pub use ascription::Ascribe;
pub use bool::{False, If, True};
pub use fix::Fix;
pub use lambda::{App, Lambda};
pub use let_exp::Let;
pub use list::{Cons, Head, IsNil, Nil, Tail};
pub use optional::{Nothing, Something};
pub use pair::{Pair, Proj1, Proj2};
pub use record::{Record, RecordProj};
pub use sum::{Left, Right, SumCase};
pub use tuple::{Proj, Tup};
pub use unit::Unit;
pub use variant::{Variant, VariantCase, VariantPattern};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Var),
    Lambda(Lambda),
    App(App),
    Unit(Unit),
    True(True),
    False(False),
    If(If),
    Ascribe(Ascribe),
    Let(Let),
    Pair(Pair),
    Proj1(Proj1),
    Proj2(Proj2),
    Tup(Tup),
    Proj(Proj),
    Record(Record),
    RecordProj(RecordProj),
    Left(Left),
    Right(Right),
    SumCase(SumCase),
    Variant(Variant),
    VariantCase(VariantCase),
    Nothing(Nothing),
    Something(Something),
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
            Term::Ascribe(asc) => asc.fmt(f),
            Term::Let(lt) => lt.fmt(f),
            Term::Pair(pr) => pr.fmt(f),
            Term::Tup(tup) => tup.fmt(f),
            Term::Proj(proj) => proj.fmt(f),
            Term::Proj1(proj) => proj.fmt(f),
            Term::Proj2(proj) => proj.fmt(f),
            Term::Record(rec) => rec.fmt(f),
            Term::RecordProj(proj) => proj.fmt(f),
            Term::Left(lf) => lf.fmt(f),
            Term::Right(rt) => rt.fmt(f),
            Term::SumCase(case) => case.fmt(f),
            Term::Variant(var) => var.fmt(f),
            Term::VariantCase(case) => case.fmt(f),
            Term::Nothing(not) => not.fmt(f),
            Term::Something(some) => some.fmt(f),
            Term::Fix(fix) => fix.fmt(f),
            Term::Nil(nil) => nil.fmt(f),
            Term::Cons(cons) => cons.fmt(f),
            Term::IsNil(isnil) => isnil.fmt(f),
            Term::Head(hd) => hd.fmt(f),
            Term::Tail(tl) => tl.fmt(f),
        }
    }
}
