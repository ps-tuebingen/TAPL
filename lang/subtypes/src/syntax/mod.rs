use crate::types::Type;
use std::fmt;

pub mod bool;
pub mod cast;
pub mod fix;
pub mod lambda;
pub mod let_exp;
pub mod list;
pub mod nat;
pub mod record;
pub mod reference;
pub mod unit;
pub mod variant;

pub use bool::{False, If, True};
pub use cast::Cast;
pub use fix::Fix;
pub use lambda::{App, Lambda};
pub use let_exp::Let;
pub use list::{Cons, ListCase, Nil};
pub use nat::{Pred, Succ, Zero};
pub use record::{Projection, Record};
pub use reference::{Assign, Deref, Location, Ref};
pub use unit::Unit;
pub use variant::{Variant, VariantCase};

pub type Label = String;
pub type Var = String;
pub type Loc = usize;

pub enum Term {
    Var(Var),
    Lambda(Lambda),
    App(App),
    Unit(Unit),
    Cast(Cast),
    Record(Record),
    Projection(Projection),
    Variant(Variant),
    VariantCase(VariantCase),
    Nil(Nil),
    Cons(Cons),
    ListCase(ListCase),
    Ref(Ref),
    Deref(Deref),
    Assign(Assign),
    Loc(Location),
    Zero(Zero),
    Succ(Succ),
    Pred(Pred),
    True(True),
    False(False),
    If(If),
    Let(Let),
    Fix(Fix),
}

impl Term {
    pub fn seq(t1: Term, t2: Term) -> Term {
        App::new(Lambda::new("_", Type::Unit, t2).into(), t1).into()
    }
}

impl From<Var> for Term {
    fn from(s: Var) -> Term {
        Term::Var(s)
    }
}

impl From<&str> for Term {
    fn from(s: &str) -> Term {
        Term::Var(s.to_owned())
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => f.write_str(v),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::Unit(unit) => unit.fmt(f),
            Term::Record(rec) => rec.fmt(f),
            Term::Projection(proj) => proj.fmt(f),
            Term::Variant(var) => var.fmt(f),
            Term::VariantCase(case) => case.fmt(f),
            Term::Cast(cast) => cast.fmt(f),
            Term::Nil(nil) => nil.fmt(f),
            Term::Cons(cons) => cons.fmt(f),
            Term::ListCase(case) => case.fmt(f),
            Term::Ref(rf) => rf.fmt(f),
            Term::Deref(deref) => deref.fmt(f),
            Term::Assign(assign) => assign.fmt(f),
            Term::Loc(loc) => loc.fmt(f),
            Term::Zero(zero) => zero.fmt(f),
            Term::Succ(succ) => succ.fmt(f),
            Term::Pred(pred) => pred.fmt(f),
            Term::True(tru) => tru.fmt(f),
            Term::False(fls) => fls.fmt(f),
            Term::If(ift) => ift.fmt(f),
            Term::Let(lt) => lt.fmt(f),
            Term::Fix(fix) => fix.fmt(f),
        }
    }
}
