use super::{types::Type, values::Value};
use common::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    terms::{
        App, False, Fix, If, IsZero, Lambda, Num, Pack, Pred, Record, RecordProj, Succ, True, Unit,
        Unpack, Variable,
    },
    TypeVar, Var,
};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<Term>),
    Unit(Unit<Term>),
    Lambda(Lambda<Term>),
    App(App<Term>),
    Pack(Pack<Term>),
    Unpack(Unpack<Term>),
    Num(Num<Term>),
    Succ(Succ<Term>),
    Pred(Pred<Term>),
    IsZero(IsZero<Term>),
    Record(Record<Term>),
    RecordProj(RecordProj<Term>),
    True(True<Term>),
    False(False<Term>),
    If(If<Term>),
    Fix(Fix<Term>),
}

impl common::terms::Term for Term {}

impl LanguageTerm for Term {
    type Type = Type;
    type Value = Value;
}

impl SubstTerm<Term> for Term {
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Unit(u) => u.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::Pack(pack) => pack.subst(v, t),
            Term::Unpack(unpack) => unpack.subst(v, t),
            Term::Num(num) => num.subst(v, t),
            Term::Succ(succ) => succ.subst(v, t),
            Term::Pred(pred) => pred.subst(v, t),
            Term::IsZero(isz) => isz.subst(v, t),
            Term::Record(rec) => rec.subst(v, t),
            Term::RecordProj(proj) => proj.subst(v, t),
            Term::True(tru) => tru.subst(v, t),
            Term::False(fls) => fls.subst(v, t),
            Term::If(ift) => ift.subst(v, t),
            Term::Fix(fix) => fix.subst(v, t),
        }
    }
}

impl SubstType<Type> for Term {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Term::Var(var) => var.subst_type(v, ty),
            Term::Unit(u) => u.subst_type(v, ty),
            Term::Lambda(lam) => lam.subst_type(v, ty),
            Term::App(app) => app.subst_type(v, ty),
            Term::Pack(pack) => pack.subst_type(v, ty),
            Term::Unpack(unpack) => unpack.subst_type(v, ty),
            Term::Num(num) => num.subst_type(v, ty),
            Term::Succ(succ) => succ.subst_type(v, ty),
            Term::Pred(pred) => pred.subst_type(v, ty),
            Term::IsZero(isz) => isz.subst_type(v, ty),
            Term::Record(rec) => rec.subst_type(v, ty),
            Term::RecordProj(proj) => proj.subst_type(v, ty),
            Term::True(tru) => tru.subst_type(v, ty),
            Term::False(fls) => fls.subst_type(v, ty),
            Term::If(ift) => ift.subst_type(v, ty),
            Term::Fix(fix) => fix.subst_type(v, ty),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => v.fmt(f),
            Term::Unit(u) => u.fmt(f),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::Pack(pack) => pack.fmt(f),
            Term::Unpack(unpack) => unpack.fmt(f),
            Term::Num(num) => num.fmt(f),
            Term::Succ(succ) => succ.fmt(f),
            Term::Pred(pred) => pred.fmt(f),
            Term::IsZero(isz) => isz.fmt(f),
            Term::Record(rec) => rec.fmt(f),
            Term::RecordProj(proj) => proj.fmt(f),
            Term::True(tru) => tru.fmt(f),
            Term::False(fls) => fls.fmt(f),
            Term::If(ift) => ift.fmt(f),
            Term::Fix(fix) => fix.fmt(f),
        }
    }
}

impl From<Pack<Term>> for Term {
    fn from(pack: Pack<Term>) -> Term {
        Term::Pack(pack)
    }
}

impl From<Unpack<Term>> for Term {
    fn from(unp: Unpack<Term>) -> Term {
        Term::Unpack(unp)
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

impl From<Record<Term>> for Term {
    fn from(rec: Record<Term>) -> Term {
        Term::Record(rec)
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

impl From<RecordProj<Term>> for Term {
    fn from(proj: RecordProj<Term>) -> Term {
        Term::RecordProj(proj)
    }
}

impl From<Fix<Term>> for Term {
    fn from(fix: Fix<Term>) -> Term {
        Term::Fix(fix)
    }
}
