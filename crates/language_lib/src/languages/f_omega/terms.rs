use super::types::Type;
use derivation::latex::LatexFmt;
use std::fmt;
use syntax::{
    subst::{SubstTerm, SubstType},
    terms::{
        App, False, Fix, If, IsZero, Lambda, Num, Pack, Pred, Record, RecordProj, Succ, True,
        TyApp, TyLambda, Unit, Unpack, Variable,
    },
    TypeVar,
};
pub type Var = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<Term>),
    Lambda(Lambda<Term, Type>),
    App(App<Term>),
    TyLambda(TyLambda<Term>),
    TyApp(TyApp<Term, Type>),
    Pack(Pack<Term, Type>),
    Unpack(Unpack<Term, Type>),
    Record(Record<Term>),
    RecordProj(RecordProj<Term>),
    True(True<Term>),
    False(False<Term>),
    If(If<Term>),
    Unit(Unit<Term>),
    Fix(Fix<Term>),
    Num(Num<Term>),
    Succ(Succ<Term>),
    Pred(Pred<Term>),
    IsZero(IsZero<Term>),
}

impl syntax::terms::Term for Term {}

impl SubstTerm<Term> for Term {
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Term {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::TyLambda(lam) => lam.subst(v, t),
            Term::TyApp(app) => app.subst(v, t),
            Term::Pack(pack) => pack.subst(v, t),
            Term::Unpack(unpack) => unpack.subst(v, t),
            Term::Record(rec) => rec.subst(v, t),
            Term::RecordProj(proj) => proj.subst(v, t),
            Term::True(tru) => tru.subst(v, t),
            Term::False(fls) => fls.subst(v, t),
            Term::If(ift) => ift.subst(v, t),
            Term::Unit(u) => u.subst(v, t),
            Term::Fix(fix) => fix.subst(v, t),
            Term::Num(num) => num.subst(v, t),
            Term::Succ(s) => s.subst(v, t),
            Term::Pred(p) => p.subst(v, t),
            Term::IsZero(isz) => isz.subst(v, t),
        }
    }
}

impl SubstType<Type> for Term {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self {
        match self {
            Term::Var(var) => var.subst_type(v, ty),
            Term::Lambda(lam) => lam.subst_type(v, ty),
            Term::App(app) => app.subst_type(v, ty),
            Term::TyLambda(lam) => lam.subst_type(v, ty),
            Term::TyApp(app) => app.subst_type(v, ty),
            Term::Pack(pack) => pack.subst_type(v, ty),
            Term::Unpack(unpack) => unpack.subst_type(v, ty),
            Term::Record(rec) => rec.subst_type(v, ty),
            Term::RecordProj(proj) => proj.subst_type(v, ty),
            Term::True(tru) => tru.subst_type(v, ty),
            Term::False(fls) => fls.subst_type(v, ty),
            Term::If(ift) => ift.subst_type(v, ty),
            Term::Unit(u) => u.subst_type(v, ty),
            Term::Fix(fix) => fix.subst_type(v, ty),
            Term::Num(num) => num.subst_type(v, ty),
            Term::Succ(s) => s.subst_type(v, ty),
            Term::Pred(p) => p.subst_type(v, ty),
            Term::IsZero(isz) => isz.subst_type(v, ty),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(var) => var.fmt(f),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::TyLambda(abs) => abs.fmt(f),
            Term::TyApp(app) => app.fmt(f),
            Term::Pack(pack) => pack.fmt(f),
            Term::Unpack(unpack) => unpack.fmt(f),
            Term::Record(rec) => rec.fmt(f),
            Term::RecordProj(proj) => proj.fmt(f),
            Term::True(tru) => tru.fmt(f),
            Term::False(fls) => fls.fmt(f),
            Term::If(ift) => ift.fmt(f),
            Term::Unit(u) => u.fmt(f),
            Term::Fix(fix) => fix.fmt(f),
            Term::Num(num) => num.fmt(f),
            Term::Succ(s) => s.fmt(f),
            Term::Pred(p) => p.fmt(f),
            Term::IsZero(isz) => isz.fmt(f),
        }
    }
}

impl LatexFmt for Term {
    fn to_latex(&self) -> String {
        match self {
            Term::Var(var) => var.to_latex(),
            Term::Lambda(lam) => lam.to_latex(),
            Term::App(app) => app.to_latex(),
            Term::TyLambda(abs) => abs.to_latex(),
            Term::TyApp(app) => app.to_latex(),
            Term::Pack(pack) => pack.to_latex(),
            Term::Unpack(unpack) => unpack.to_latex(),
            Term::Record(rec) => rec.to_latex(),
            Term::RecordProj(proj) => proj.to_latex(),
            Term::True(tru) => tru.to_latex(),
            Term::False(fls) => fls.to_latex(),
            Term::If(ift) => ift.to_latex(),
            Term::Unit(u) => u.to_latex(),
            Term::Fix(fix) => fix.to_latex(),
            Term::Num(num) => num.to_latex(),
            Term::Succ(s) => s.to_latex(),
            Term::Pred(p) => p.to_latex(),
            Term::IsZero(isz) => isz.to_latex(),
        }
    }
}

impl From<Pack<Term, Type>> for Term {
    fn from(pack: Pack<Term, Type>) -> Term {
        Term::Pack(pack)
    }
}
impl From<Unpack<Term, Type>> for Term {
    fn from(unpack: Unpack<Term, Type>) -> Term {
        Term::Unpack(unpack)
    }
}
impl From<TyApp<Term, Type>> for Term {
    fn from(tyapp: TyApp<Term, Type>) -> Term {
        Term::TyApp(tyapp)
    }
}

impl From<TyLambda<Term>> for Term {
    fn from(tylam: TyLambda<Term>) -> Term {
        Term::TyLambda(tylam)
    }
}
impl From<Lambda<Term, Type>> for Term {
    fn from(lam: Lambda<Term, Type>) -> Term {
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
