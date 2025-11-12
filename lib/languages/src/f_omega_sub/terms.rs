use super::{FOmegaSub, types::Type};
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use macros::Typecheck;
use std::fmt;
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{
        App, Lambda, LambdaSub, Let, Num, Pack, Pred, Record, RecordProj, Succ, TyApp, Unpack,
        Variable,
    },
};

#[derive(Typecheck, Debug, Clone, PartialEq, Eq)]
#[Lang(FOmegaSub)]
pub enum Term {
    Var(Variable<FOmegaSub>),
    Lambda(Lambda<FOmegaSub>),
    App(App<FOmegaSub>),
    LambdaSub(LambdaSub<FOmegaSub>),
    TyApp(TyApp<FOmegaSub>),
    Pack(Pack<FOmegaSub>),
    Unpack(Unpack<FOmegaSub>),
    Record(Record<FOmegaSub>),
    RecordProj(RecordProj<FOmegaSub>),
    Num(Num<FOmegaSub>),
    Succ(Succ<FOmegaSub>),
    Pred(Pred<FOmegaSub>),
    Let(Let<FOmegaSub>),
}

impl syntax::terms::Term for Term {}

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            Variable::<FOmegaSub>::rule(),
            Lambda::<FOmegaSub>::rule(),
            App::<FOmegaSub>::rule(),
            LambdaSub::<FOmegaSub>::rule(),
            TyApp::<FOmegaSub>::rule(),
            Pack::<FOmegaSub>::rule(),
            Unpack::<FOmegaSub>::rule(),
            Record::<FOmegaSub>::rule(),
            RecordProj::<FOmegaSub>::rule(),
            Num::<FOmegaSub>::rule(),
            Succ::<FOmegaSub>::rule(),
            Pred::<FOmegaSub>::rule(),
            Let::<FOmegaSub>::rule(),
        ])
    }
}

impl SubstTerm for Term {
    type Lang = FOmegaSub;
    type Target = Self;

    fn subst(self, v: &Var, t: &Term) -> Term {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t).into(),
            Term::App(app) => app.subst(v, t).into(),
            Term::LambdaSub(lam) => lam.subst(v, t).into(),
            Term::TyApp(app) => app.subst(v, t).into(),
            Term::Pack(pack) => pack.subst(v, t).into(),
            Term::Unpack(unpack) => unpack.subst(v, t).into(),
            Term::Record(rec) => rec.subst(v, t).into(),
            Term::RecordProj(proj) => proj.subst(v, t).into(),
            Term::Num(num) => num.subst(v, t).into(),
            Term::Succ(succ) => succ.subst(v, t).into(),
            Term::Pred(pred) => pred.subst(v, t).into(),
            Term::Let(lt) => lt.subst(v, t).into(),
        }
    }
}

impl SubstType for Term {
    type Lang = FOmegaSub;
    type Target = Self;

    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self {
        match self {
            Term::Var(var) => var.subst_type(v, ty).into(),
            Term::Lambda(lam) => lam.subst_type(v, ty).into(),
            Term::App(app) => app.subst_type(v, ty).into(),
            Term::LambdaSub(lam) => lam.subst_type(v, ty).into(),
            Term::TyApp(app) => app.subst_type(v, ty).into(),
            Term::Pack(pack) => pack.subst_type(v, ty).into(),
            Term::Unpack(unpack) => unpack.subst_type(v, ty).into(),
            Term::Record(rec) => rec.subst_type(v, ty).into(),
            Term::RecordProj(proj) => proj.subst_type(v, ty).into(),
            Term::Num(num) => num.subst_type(v, ty).into(),
            Term::Succ(succ) => succ.subst_type(v, ty).into(),
            Term::Pred(pred) => pred.subst_type(v, ty).into(),
            Term::Let(lt) => lt.subst_type(v, ty).into(),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(var) => var.fmt(f),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::LambdaSub(lam) => lam.fmt(f),
            Term::TyApp(app) => app.fmt(f),
            Term::Pack(pack) => pack.fmt(f),
            Term::Unpack(unpack) => unpack.fmt(f),
            Term::Record(rec) => rec.fmt(f),
            Term::RecordProj(proj) => proj.fmt(f),
            Term::Num(num) => num.fmt(f),
            Term::Succ(succ) => succ.fmt(f),
            Term::Pred(pred) => pred.fmt(f),
            Term::Let(lt) => lt.fmt(f),
        }
    }
}

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Var(var) => var.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::LambdaSub(lam) => lam.to_latex(conf),
            Term::TyApp(app) => app.to_latex(conf),
            Term::Pack(pack) => pack.to_latex(conf),
            Term::Unpack(unpack) => unpack.to_latex(conf),
            Term::Record(rec) => rec.to_latex(conf),
            Term::RecordProj(proj) => proj.to_latex(conf),
            Term::Num(num) => num.to_latex(conf),
            Term::Succ(succ) => succ.to_latex(conf),
            Term::Pred(pred) => pred.to_latex(conf),
            Term::Let(lt) => lt.to_latex(conf),
        }
    }
}

impl From<Let<FOmegaSub>> for Term {
    fn from(lt: Let<FOmegaSub>) -> Term {
        Term::Let(lt)
    }
}
impl From<Pack<FOmegaSub>> for Term {
    fn from(pack: Pack<FOmegaSub>) -> Term {
        Term::Pack(pack)
    }
}
impl From<Unpack<FOmegaSub>> for Term {
    fn from(unpack: Unpack<FOmegaSub>) -> Term {
        Term::Unpack(unpack)
    }
}
impl From<TyApp<FOmegaSub>> for Term {
    fn from(tyapp: TyApp<FOmegaSub>) -> Term {
        Term::TyApp(tyapp)
    }
}

impl From<Lambda<FOmegaSub>> for Term {
    fn from(lam: Lambda<FOmegaSub>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<Num<FOmegaSub>> for Term {
    fn from(num: Num<FOmegaSub>) -> Term {
        Term::Num(num)
    }
}

impl From<Record<FOmegaSub>> for Term {
    fn from(rec: Record<FOmegaSub>) -> Term {
        Term::Record(rec)
    }
}

impl From<Variable<FOmegaSub>> for Term {
    fn from(var: Variable<FOmegaSub>) -> Term {
        Term::Var(var)
    }
}

impl From<App<FOmegaSub>> for Term {
    fn from(app: App<FOmegaSub>) -> Term {
        Term::App(app)
    }
}

impl From<Pred<FOmegaSub>> for Term {
    fn from(pred: Pred<FOmegaSub>) -> Term {
        Term::Pred(pred)
    }
}

impl From<Succ<FOmegaSub>> for Term {
    fn from(succ: Succ<FOmegaSub>) -> Term {
        Term::Succ(succ)
    }
}
impl From<LambdaSub<FOmegaSub>> for Term {
    fn from(lam: LambdaSub<FOmegaSub>) -> Term {
        Term::LambdaSub(lam)
    }
}
impl From<RecordProj<FOmegaSub>> for Term {
    fn from(proj: RecordProj<FOmegaSub>) -> Term {
        Term::RecordProj(proj)
    }
}
