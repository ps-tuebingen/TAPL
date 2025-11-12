use super::{FOmegaSub, types::Type};
use latex::{LatexConfig, LatexFmt};
use macros::{Eval, GrammarDescribe, TermDisplay, Typecheck};
use std::fmt;
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{
        App, Lambda, LambdaSub, Let, Num, Pack, Pred, Record, RecordProj, Succ, TyApp, Unpack,
        Variable,
    },
};

#[derive(TermDisplay, GrammarDescribe, Eval, Typecheck, Debug, Clone, PartialEq, Eq)]
#[Lang(FOmegaSub)]
pub enum Term {
    Variable(Variable<FOmegaSub>),
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

impl SubstTerm for Term {
    type Lang = FOmegaSub;
    type Target = Self;

    fn subst(self, v: &Var, t: &Term) -> Term {
        match self {
            Term::Variable(var) => var.subst(v, t),
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
            Term::Variable(var) => var.subst_type(v, ty).into(),
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

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Variable(var) => var.to_latex(conf),
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
        Term::Variable(var)
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
