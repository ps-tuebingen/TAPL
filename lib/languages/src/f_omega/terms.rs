use super::{FOmega, types::Type};
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar,
    subst::{SubstTerm, SubstType},
    terms::{
        App, False, Fix, If, IsZero, Lambda, Num, Pack, Pred, Record, RecordProj, Succ, True,
        TyApp, TyLambda, Unit, Unpack, Variable,
    },
};
pub type Var = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<FOmega>),
    Lambda(Lambda<FOmega>),
    App(App<FOmega>),
    TyLambda(TyLambda<FOmega>),
    TyApp(TyApp<FOmega>),
    Pack(Pack<FOmega>),
    Unpack(Unpack<FOmega>),
    Record(Record<FOmega>),
    RecordProj(RecordProj<FOmega>),
    True(True<FOmega>),
    False(False<FOmega>),
    If(If<FOmega>),
    Unit(Unit<FOmega>),
    Fix(Fix<FOmega>),
    Num(Num<FOmega>),
    Succ(Succ<FOmega>),
    Pred(Pred<FOmega>),
    IsZero(IsZero<FOmega>),
}

impl syntax::terms::Term for Term {}

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            Variable::<FOmega>::rule(),
            Lambda::<FOmega>::rule(),
            App::<FOmega>::rule(),
            TyLambda::<FOmega>::rule(),
            TyApp::<FOmega>::rule(),
            Pack::<FOmega>::rule(),
            Unpack::<FOmega>::rule(),
            Record::<FOmega>::rule(),
            RecordProj::<FOmega>::rule(),
            True::<FOmega>::rule(),
            False::<FOmega>::rule(),
            If::<FOmega>::rule(),
            Unit::<FOmega>::rule(),
            Fix::<FOmega>::rule(),
            Num::<FOmega>::rule(),
            Succ::<FOmega>::rule(),
            Pred::<FOmega>::rule(),
            IsZero::<FOmega>::rule(),
        ])
    }
}

impl SubstTerm for Term {
    type Lang = FOmega;
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Term {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t).into(),
            Term::App(app) => app.subst(v, t).into(),
            Term::TyLambda(lam) => lam.subst(v, t).into(),
            Term::TyApp(app) => app.subst(v, t).into(),
            Term::Pack(pack) => pack.subst(v, t).into(),
            Term::Unpack(unpack) => unpack.subst(v, t).into(),
            Term::Record(rec) => rec.subst(v, t).into(),
            Term::RecordProj(proj) => proj.subst(v, t).into(),
            Term::True(tru) => tru.subst(v, t).into(),
            Term::False(fls) => fls.subst(v, t).into(),
            Term::If(ift) => ift.subst(v, t).into(),
            Term::Unit(u) => u.subst(v, t).into(),
            Term::Fix(fix) => fix.subst(v, t).into(),
            Term::Num(num) => num.subst(v, t).into(),
            Term::Succ(s) => s.subst(v, t).into(),
            Term::Pred(p) => p.subst(v, t).into(),
            Term::IsZero(isz) => isz.subst(v, t).into(),
        }
    }
}

impl SubstType for Term {
    type Lang = FOmega;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self {
        match self {
            Term::Var(var) => var.subst_type(v, ty).into(),
            Term::Lambda(lam) => lam.subst_type(v, ty).into(),
            Term::App(app) => app.subst_type(v, ty).into(),
            Term::TyLambda(lam) => lam.subst_type(v, ty).into(),
            Term::TyApp(app) => app.subst_type(v, ty).into(),
            Term::Pack(pack) => pack.subst_type(v, ty).into(),
            Term::Unpack(unpack) => unpack.subst_type(v, ty).into(),
            Term::Record(rec) => rec.subst_type(v, ty).into(),
            Term::RecordProj(proj) => proj.subst_type(v, ty).into(),
            Term::True(tru) => tru.subst_type(v, ty).into(),
            Term::False(fls) => fls.subst_type(v, ty).into(),
            Term::If(ift) => ift.subst_type(v, ty).into(),
            Term::Unit(u) => u.subst_type(v, ty).into(),
            Term::Fix(fix) => fix.subst_type(v, ty).into(),
            Term::Num(num) => num.subst_type(v, ty).into(),
            Term::Succ(s) => s.subst_type(v, ty).into(),
            Term::Pred(p) => p.subst_type(v, ty).into(),
            Term::IsZero(isz) => isz.subst_type(v, ty).into(),
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
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Var(var) => var.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::TyLambda(abs) => abs.to_latex(conf),
            Term::TyApp(app) => app.to_latex(conf),
            Term::Pack(pack) => pack.to_latex(conf),
            Term::Unpack(unpack) => unpack.to_latex(conf),
            Term::Record(rec) => rec.to_latex(conf),
            Term::RecordProj(proj) => proj.to_latex(conf),
            Term::True(tru) => tru.to_latex(conf),
            Term::False(fls) => fls.to_latex(conf),
            Term::If(ift) => ift.to_latex(conf),
            Term::Unit(u) => u.to_latex(conf),
            Term::Fix(fix) => fix.to_latex(conf),
            Term::Num(num) => num.to_latex(conf),
            Term::Succ(s) => s.to_latex(conf),
            Term::Pred(p) => p.to_latex(conf),
            Term::IsZero(isz) => isz.to_latex(conf),
        }
    }
}

impl From<Pack<FOmega>> for Term {
    fn from(pack: Pack<FOmega>) -> Term {
        Term::Pack(pack)
    }
}
impl From<Unpack<FOmega>> for Term {
    fn from(unpack: Unpack<FOmega>) -> Term {
        Term::Unpack(unpack)
    }
}
impl From<TyApp<FOmega>> for Term {
    fn from(tyapp: TyApp<FOmega>) -> Term {
        Term::TyApp(tyapp)
    }
}

impl From<TyLambda<FOmega>> for Term {
    fn from(tylam: TyLambda<FOmega>) -> Term {
        Term::TyLambda(tylam)
    }
}
impl From<Lambda<FOmega>> for Term {
    fn from(lam: Lambda<FOmega>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<Unit<FOmega>> for Term {
    fn from(u: Unit<FOmega>) -> Term {
        Term::Unit(u)
    }
}

impl From<True<FOmega>> for Term {
    fn from(tru: True<FOmega>) -> Term {
        Term::True(tru)
    }
}

impl From<False<FOmega>> for Term {
    fn from(fls: False<FOmega>) -> Term {
        Term::False(fls)
    }
}

impl From<Num<FOmega>> for Term {
    fn from(num: Num<FOmega>) -> Term {
        Term::Num(num)
    }
}

impl From<Record<FOmega>> for Term {
    fn from(rec: Record<FOmega>) -> Term {
        Term::Record(rec)
    }
}

impl From<Variable<FOmega>> for Term {
    fn from(var: Variable<FOmega>) -> Term {
        Term::Var(var)
    }
}

impl From<App<FOmega>> for Term {
    fn from(app: App<FOmega>) -> Term {
        Term::App(app)
    }
}

impl From<If<FOmega>> for Term {
    fn from(ift: If<FOmega>) -> Term {
        Term::If(ift)
    }
}

impl From<Pred<FOmega>> for Term {
    fn from(pred: Pred<FOmega>) -> Term {
        Term::Pred(pred)
    }
}

impl From<Succ<FOmega>> for Term {
    fn from(succ: Succ<FOmega>) -> Term {
        Term::Succ(succ)
    }
}

impl From<IsZero<FOmega>> for Term {
    fn from(isz: IsZero<FOmega>) -> Term {
        Term::IsZero(isz)
    }
}

impl From<RecordProj<FOmega>> for Term {
    fn from(proj: RecordProj<FOmega>) -> Term {
        Term::RecordProj(proj)
    }
}

impl From<Fix<FOmega>> for Term {
    fn from(fix: Fix<FOmega>) -> Term {
        Term::Fix(fix)
    }
}
