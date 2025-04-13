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
pub use nat::{IsZero, Pred, Succ, Zero};
pub use optional::{Nothing, SomeCase, Something};
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
    Zero(Zero),
    Pred(Pred),
    Succ(Succ),
    IsZero(IsZero),
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
    SomeCase(SomeCase),
    Fix(Fix),
    Nil(Nil),
    Cons(Cons),
    IsNil(IsNil),
    Head(Head),
    Tail(Tail),
}

impl Term {
    pub fn is_numeric_value(&self) -> bool {
        match self {
            Term::Zero(_) => true,
            Term::Succ(s) => s.term.is_numeric_value() && !matches!(*s.term, Term::Pred(_)),
            Term::Pred(p) => p.term.is_numeric_value() && !matches!(*p.term, Term::Succ(_)),
            _ => false,
        }
    }
    pub fn is_value(&self) -> bool {
        match self {
            Term::Var(_) => false,
            Term::Lambda(_) => true,
            Term::App(_) => false,
            Term::Unit(_) => true,
            Term::True(_) => true,
            Term::False(_) => true,
            Term::If(_) => false,
            Term::Zero(_) => true,
            Term::Pred(_) => self.is_numeric_value(),
            Term::Succ(_) => self.is_numeric_value(),
            Term::IsZero(_) => false,
            Term::Ascribe(asc) => asc.term.is_value(),
            Term::Let(_) => false,
            Term::Pair(p) => p.fst.is_value() && p.snd.is_value(),
            Term::Proj1(_) => false,
            Term::Proj2(_) => false,
            Term::Tup(tups) => tups.terms.iter().all(|x| x.is_value()),
            Term::Proj(_) => false,
            Term::Record(rec) => rec.records.iter().all(|(_, t)| t.is_value()),
            Term::RecordProj(_) => false,
            Term::Left(l) => l.left_term.is_value(),
            Term::Right(r) => r.right_term.is_value(),
            Term::SumCase(_) => false,
            Term::Variant(var) => var.term.is_value(),
            Term::VariantCase(_) => false,
            Term::Nothing(_) => true,
            Term::Something(s) => s.term.is_value(),
            Term::SomeCase(_) => false,
            Term::Fix(_) => false,
            Term::Nil(_) => true,
            Term::Cons(c) => c.fst.is_value() && c.rst.is_value(),
            Term::IsNil(_) => false,
            Term::Head(_) => false,
            Term::Tail(_) => false,
        }
    }
}

impl From<Var> for Term {
    fn from(var: Var) -> Term {
        Term::Var(var)
    }
}

impl From<i64> for Term {
    fn from(i: i64) -> Term {
        match i {
            0 => Zero.into(),
            i if i > 0 => {
                let pred: Term = (i - 1).into();
                Succ {
                    term: Box::new(pred),
                }
                .into()
            }
            _ => {
                let succ = (i + 1).into();
                Pred {
                    term: Box::new(succ),
                }
                .into()
            }
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
            Term::Zero(z) => z.fmt(f),
            Term::Pred(p) => p.fmt(f),
            Term::Succ(s) => s.fmt(f),
            Term::IsZero(isz) => isz.fmt(f),
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
