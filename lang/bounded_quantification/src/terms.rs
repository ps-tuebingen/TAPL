use crate::{types::Type, values::Value};
use common::{
    language::LanguageTerm,
    subst::{SubstTerm, SubstType},
    terms::{
        App, Lambda, LambdaSub, Num, Pack, Pred, Projection, Record, Succ, TyApp, Unpack, Variable,
    },
    TypeVar, Var,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Var(Variable<Term>),
    Num(Num<Term>),
    Succ(Succ<Term>),
    Pred(Pred<Term>),
    Lambda(Lambda<Term>),
    App(App<Term>),
    LambdaSub(LambdaSub<Term>),
    TyApp(TyApp<Term>),
    Pack(Pack<Term>),
    Unpack(Unpack<Term>),
    Record(Record<Term>),
    Projection(Projection<Term>),
}

impl common::terms::Term for Term {}

impl LanguageTerm for Term {
    type Type = Type;
    type Value = Value;
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
            Term::Projection(proj) => proj.fmt(f),
        }
    }
}

impl SubstTerm<Term> for Term {
    type Target = Term;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Num(num) => num.subst(v, t),
            Term::Succ(succ) => succ.subst(v, t),
            Term::Pred(pred) => pred.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::LambdaSub(lam) => lam.subst(v, t),
            Term::TyApp(app) => app.subst(v, t),
            Term::Pack(pack) => pack.subst(v, t),
            Term::Unpack(unpack) => unpack.subst(v, t),
            Term::Record(rec) => rec.subst(v, t),
            Term::Projection(proj) => proj.subst(v, t),
        }
    }
}

impl SubstType<Type> for Term {
    type Target = Term;
    fn subst_type(self, v: &TypeVar, t: &Type) -> Self::Target {
        match self {
            Term::Var(var) => var.subst_type(v, t),
            Term::Num(num) => num.subst_type(v, t),
            Term::Succ(succ) => succ.subst_type(v, t),
            Term::Pred(pred) => pred.subst_type(v, t),
            Term::Lambda(lam) => lam.subst_type(v, t),
            Term::App(app) => app.subst_type(v, t),
            Term::LambdaSub(lam) => lam.subst_type(v, t),
            Term::TyApp(app) => app.subst_type(v, t),
            Term::Pack(pack) => pack.subst_type(v, t),
            Term::Unpack(unpack) => unpack.subst_type(v, t),
            Term::Record(rec) => rec.subst_type(v, t),
            Term::Projection(proj) => proj.subst_type(v, t),
        }
    }
}

impl From<Variable<Term>> for Term {
    fn from(var: Variable<Term>) -> Term {
        Term::Var(var)
    }
}

impl From<Num<Term>> for Term {
    fn from(num: Num<Term>) -> Term {
        Term::Num(num)
    }
}

impl From<Succ<Term>> for Term {
    fn from(succ: Succ<Term>) -> Term {
        Term::Succ(succ)
    }
}

impl From<Pred<Term>> for Term {
    fn from(pred: Pred<Term>) -> Term {
        Term::Pred(pred)
    }
}

impl From<Lambda<Term>> for Term {
    fn from(lam: Lambda<Term>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<App<Term>> for Term {
    fn from(app: App<Term>) -> Term {
        Term::App(app)
    }
}

impl From<LambdaSub<Term>> for Term {
    fn from(lam: LambdaSub<Term>) -> Term {
        Term::LambdaSub(lam)
    }
}

impl From<TyApp<Term>> for Term {
    fn from(app: TyApp<Term>) -> Term {
        Term::TyApp(app)
    }
}

impl From<Pack<Term>> for Term {
    fn from(pack: Pack<Term>) -> Term {
        Term::Pack(pack)
    }
}

impl From<Unpack<Term>> for Term {
    fn from(unpack: Unpack<Term>) -> Term {
        Term::Unpack(unpack)
    }
}

impl From<Record<Term>> for Term {
    fn from(rec: Record<Term>) -> Term {
        Term::Record(rec)
    }
}

impl From<Projection<Term>> for Term {
    fn from(proj: Projection<Term>) -> Term {
        Term::Projection(proj)
    }
}
