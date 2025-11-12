use super::{Existential, types::Type};
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use macros::Typecheck;
use std::fmt;
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{
        App, False, Fix, If, IsZero, Lambda, Num, Pack, Pred, Record, RecordProj, Succ, True, Unit,
        Unpack, Variable,
    },
};

#[derive(Typecheck, Debug, Clone, PartialEq, Eq)]
#[Lang(Existential)]
pub enum Term {
    Var(Variable<Existential>),
    Unit(Unit<Existential>),
    Lambda(Lambda<Existential>),
    App(App<Existential>),
    Pack(Pack<Existential>),
    Unpack(Unpack<Existential>),
    Num(Num<Existential>),
    Succ(Succ<Existential>),
    Pred(Pred<Existential>),
    IsZero(IsZero<Existential>),
    Record(Record<Existential>),
    RecordProj(RecordProj<Existential>),
    True(True<Existential>),
    False(False<Existential>),
    If(If<Existential>),
    Fix(Fix<Existential>),
}

impl syntax::terms::Term for Term {}

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            Variable::<Existential>::rule(),
            Unit::<Existential>::rule(),
            Lambda::<Existential>::rule(),
            App::<Existential>::rule(),
            Pack::<Existential>::rule(),
            Unpack::<Existential>::rule(),
            Num::<Existential>::rule(),
            Succ::<Existential>::rule(),
            Pred::<Existential>::rule(),
            IsZero::<Existential>::rule(),
            Record::<Existential>::rule(),
            RecordProj::<Existential>::rule(),
            True::<Existential>::rule(),
            False::<Existential>::rule(),
            If::<Existential>::rule(),
            Fix::<Existential>::rule(),
        ])
    }
}

impl SubstTerm for Term {
    type Lang = Existential;
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Unit(u) => u.subst(v, t).into(),
            Term::Lambda(lam) => lam.subst(v, t).into(),
            Term::App(app) => app.subst(v, t).into(),
            Term::Pack(pack) => pack.subst(v, t).into(),
            Term::Unpack(unpack) => unpack.subst(v, t).into(),
            Term::Num(num) => num.subst(v, t).into(),
            Term::Succ(succ) => succ.subst(v, t).into(),
            Term::Pred(pred) => pred.subst(v, t).into(),
            Term::IsZero(isz) => isz.subst(v, t).into(),
            Term::Record(rec) => rec.subst(v, t).into(),
            Term::RecordProj(proj) => proj.subst(v, t).into(),
            Term::True(tru) => tru.subst(v, t).into(),
            Term::False(fls) => fls.subst(v, t).into(),
            Term::If(ift) => ift.subst(v, t).into(),
            Term::Fix(fix) => fix.subst(v, t).into(),
        }
    }
}

impl SubstType for Term {
    type Lang = Existential;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Term::Var(var) => var.subst_type(v, ty).into(),
            Term::Unit(u) => u.subst_type(v, ty).into(),
            Term::Lambda(lam) => lam.subst_type(v, ty).into(),
            Term::App(app) => app.subst_type(v, ty).into(),
            Term::Pack(pack) => pack.subst_type(v, ty).into(),
            Term::Unpack(unpack) => unpack.subst_type(v, ty).into(),
            Term::Num(num) => num.subst_type(v, ty).into(),
            Term::Succ(succ) => succ.subst_type(v, ty).into(),
            Term::Pred(pred) => pred.subst_type(v, ty).into(),
            Term::IsZero(isz) => isz.subst_type(v, ty).into(),
            Term::Record(rec) => rec.subst_type(v, ty).into(),
            Term::RecordProj(proj) => proj.subst_type(v, ty).into(),
            Term::True(tru) => tru.subst_type(v, ty).into(),
            Term::False(fls) => fls.subst_type(v, ty).into(),
            Term::If(ift) => ift.subst_type(v, ty).into(),
            Term::Fix(fix) => fix.subst_type(v, ty).into(),
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

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Var(v) => v.to_latex(conf),
            Term::Unit(u) => u.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::Pack(pack) => pack.to_latex(conf),
            Term::Unpack(unpack) => unpack.to_latex(conf),
            Term::Num(num) => num.to_latex(conf),
            Term::Succ(succ) => succ.to_latex(conf),
            Term::Pred(pred) => pred.to_latex(conf),
            Term::IsZero(isz) => isz.to_latex(conf),
            Term::Record(rec) => rec.to_latex(conf),
            Term::RecordProj(proj) => proj.to_latex(conf),
            Term::True(tru) => tru.to_latex(conf),
            Term::False(fls) => fls.to_latex(conf),
            Term::If(ift) => ift.to_latex(conf),
            Term::Fix(fix) => fix.to_latex(conf),
        }
    }
}
impl From<Pack<Existential>> for Term {
    fn from(pack: Pack<Existential>) -> Term {
        Term::Pack(pack)
    }
}

impl From<Unpack<Existential>> for Term {
    fn from(unp: Unpack<Existential>) -> Term {
        Term::Unpack(unp)
    }
}
impl From<Lambda<Existential>> for Term {
    fn from(lam: Lambda<Existential>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<Unit<Existential>> for Term {
    fn from(u: Unit<Existential>) -> Term {
        Term::Unit(u)
    }
}

impl From<True<Existential>> for Term {
    fn from(tru: True<Existential>) -> Term {
        Term::True(tru)
    }
}

impl From<False<Existential>> for Term {
    fn from(fls: False<Existential>) -> Term {
        Term::False(fls)
    }
}

impl From<Num<Existential>> for Term {
    fn from(num: Num<Existential>) -> Term {
        Term::Num(num)
    }
}

impl From<Record<Existential>> for Term {
    fn from(rec: Record<Existential>) -> Term {
        Term::Record(rec)
    }
}

impl From<Variable<Existential>> for Term {
    fn from(var: Variable<Existential>) -> Term {
        Term::Var(var)
    }
}

impl From<App<Existential>> for Term {
    fn from(app: App<Existential>) -> Term {
        Term::App(app)
    }
}

impl From<If<Existential>> for Term {
    fn from(ift: If<Existential>) -> Term {
        Term::If(ift)
    }
}

impl From<Pred<Existential>> for Term {
    fn from(pred: Pred<Existential>) -> Term {
        Term::Pred(pred)
    }
}

impl From<Succ<Existential>> for Term {
    fn from(succ: Succ<Existential>) -> Term {
        Term::Succ(succ)
    }
}

impl From<IsZero<Existential>> for Term {
    fn from(isz: IsZero<Existential>) -> Term {
        Term::IsZero(isz)
    }
}

impl From<RecordProj<Existential>> for Term {
    fn from(proj: RecordProj<Existential>) -> Term {
        Term::RecordProj(proj)
    }
}

impl From<Fix<Existential>> for Term {
    fn from(fix: Fix<Existential>) -> Term {
        Term::Fix(fix)
    }
}
