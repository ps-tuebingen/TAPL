use super::{BoundedQuantification, types::Type};
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use macros::Typecheck;
use std::fmt;
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{
        App, Lambda, LambdaSub, Num, Pack, Pred, Record, RecordProj, Succ, TyApp, Unpack, Variable,
    },
};

#[derive(Typecheck, Clone, Debug, PartialEq, Eq)]
#[Lang(BoundedQuantification)]
pub enum Term {
    Var(Variable<BoundedQuantification>),
    Num(Num<BoundedQuantification>),
    Succ(Succ<BoundedQuantification>),
    Pred(Pred<BoundedQuantification>),
    Lambda(Lambda<BoundedQuantification>),
    App(App<BoundedQuantification>),
    LambdaSub(LambdaSub<BoundedQuantification>),
    TyApp(TyApp<BoundedQuantification>),
    Pack(Pack<BoundedQuantification>),
    Unpack(Unpack<BoundedQuantification>),
    Record(Record<BoundedQuantification>),
    RecordProj(RecordProj<BoundedQuantification>),
}

impl syntax::terms::Term for Term {}

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            Variable::<BoundedQuantification>::rule(),
            Num::<BoundedQuantification>::rule(),
            Succ::<BoundedQuantification>::rule(),
            Pred::<BoundedQuantification>::rule(),
            Lambda::<BoundedQuantification>::rule(),
            App::<BoundedQuantification>::rule(),
            LambdaSub::<BoundedQuantification>::rule(),
            TyApp::<BoundedQuantification>::rule(),
            Pack::<BoundedQuantification>::rule(),
            Unpack::<BoundedQuantification>::rule(),
            Record::<BoundedQuantification>::rule(),
            RecordProj::<BoundedQuantification>::rule(),
        ])
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => v.fmt(f),
            Term::Num(num) => num.fmt(f),
            Term::Succ(s) => s.fmt(f),
            Term::Pred(p) => p.fmt(f),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::LambdaSub(lam) => lam.fmt(f),
            Term::TyApp(app) => app.fmt(f),
            Term::Pack(pack) => pack.fmt(f),
            Term::Unpack(unpack) => unpack.fmt(f),
            Term::Record(rec) => rec.fmt(f),
            Term::RecordProj(proj) => proj.fmt(f),
        }
    }
}

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Var(v) => v.to_latex(conf),
            Term::Num(num) => num.to_latex(conf),
            Term::Succ(s) => s.to_latex(conf),
            Term::Pred(p) => p.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::LambdaSub(lam) => lam.to_latex(conf),
            Term::TyApp(app) => app.to_latex(conf),
            Term::Pack(pack) => pack.to_latex(conf),
            Term::Unpack(unpack) => unpack.to_latex(conf),
            Term::Record(rec) => rec.to_latex(conf),
            Term::RecordProj(proj) => proj.to_latex(conf),
        }
    }
}

impl SubstTerm for Term {
    type Lang = BoundedQuantification;
    type Target = Term;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Num(num) => num.subst(v, t).into(),
            Term::Succ(succ) => succ.subst(v, t).into(),
            Term::Pred(pred) => pred.subst(v, t).into(),
            Term::Lambda(lam) => lam.subst(v, t).into(),
            Term::App(app) => app.subst(v, t).into(),
            Term::LambdaSub(lam) => lam.subst(v, t).into(),
            Term::TyApp(app) => app.subst(v, t).into(),
            Term::Pack(pack) => pack.subst(v, t).into(),
            Term::Unpack(unpack) => unpack.subst(v, t).into(),
            Term::Record(rec) => rec.subst(v, t).into(),
            Term::RecordProj(proj) => proj.subst(v, t).into(),
        }
    }
}

impl SubstType for Term {
    type Lang = BoundedQuantification;
    type Target = Term;
    fn subst_type(self, v: &TypeVar, t: &Type) -> Self::Target {
        match self {
            Term::Var(var) => var.subst_type(v, t).into(),
            Term::Num(num) => num.subst_type(v, t).into(),
            Term::Succ(succ) => succ.subst_type(v, t).into(),
            Term::Pred(pred) => pred.subst_type(v, t).into(),
            Term::Lambda(lam) => lam.subst_type(v, t).into(),
            Term::App(app) => app.subst_type(v, t).into(),
            Term::LambdaSub(lam) => lam.subst_type(v, t).into(),
            Term::TyApp(app) => app.subst_type(v, t).into(),
            Term::Pack(pack) => pack.subst_type(v, t).into(),
            Term::Unpack(unpack) => unpack.subst_type(v, t).into(),
            Term::Record(rec) => rec.subst_type(v, t).into(),
            Term::RecordProj(proj) => proj.subst_type(v, t).into(),
        }
    }
}

impl From<Variable<BoundedQuantification>> for Term {
    fn from(var: Variable<BoundedQuantification>) -> Term {
        Term::Var(var)
    }
}

impl From<Num<BoundedQuantification>> for Term {
    fn from(num: Num<BoundedQuantification>) -> Term {
        Term::Num(num)
    }
}

impl From<Succ<BoundedQuantification>> for Term {
    fn from(succ: Succ<BoundedQuantification>) -> Term {
        Term::Succ(succ)
    }
}

impl From<Pred<BoundedQuantification>> for Term {
    fn from(pred: Pred<BoundedQuantification>) -> Term {
        Term::Pred(pred)
    }
}

impl From<Lambda<BoundedQuantification>> for Term {
    fn from(lam: Lambda<BoundedQuantification>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<App<BoundedQuantification>> for Term {
    fn from(app: App<BoundedQuantification>) -> Term {
        Term::App(app)
    }
}

impl From<LambdaSub<BoundedQuantification>> for Term {
    fn from(lam: LambdaSub<BoundedQuantification>) -> Term {
        Term::LambdaSub(lam)
    }
}

impl From<TyApp<BoundedQuantification>> for Term {
    fn from(app: TyApp<BoundedQuantification>) -> Term {
        Term::TyApp(app)
    }
}

impl From<Pack<BoundedQuantification>> for Term {
    fn from(pack: Pack<BoundedQuantification>) -> Term {
        Term::Pack(pack)
    }
}

impl From<Unpack<BoundedQuantification>> for Term {
    fn from(unpack: Unpack<BoundedQuantification>) -> Term {
        Term::Unpack(unpack)
    }
}

impl From<Record<BoundedQuantification>> for Term {
    fn from(rec: Record<BoundedQuantification>) -> Term {
        Term::Record(rec)
    }
}

impl From<RecordProj<BoundedQuantification>> for Term {
    fn from(proj: RecordProj<BoundedQuantification>) -> Term {
        Term::RecordProj(proj)
    }
}
