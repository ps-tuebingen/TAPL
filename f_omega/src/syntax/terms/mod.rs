use crate::{
    syntax::types::{Type, TypeVar},
    traits::{SubstTerm, SubstTy},
};
use std::fmt;
pub type Var = String;

pub mod app;
pub mod lambda;
pub mod pack;
pub mod record;
pub mod record_proj;
pub mod tyapp;
pub mod tylambda;
pub mod unpack;
pub use app::App;
pub use lambda::Lambda;
pub use pack::Pack;
pub use record::Record;
pub use record_proj::RecordProj;
pub use tyapp::TyApp;
pub use tylambda::TyLambda;
pub use unpack::Unpack;

#[derive(Debug, Clone)]
pub enum Term {
    Var(Var),
    Lambda(Lambda),
    App(App),
    TyLambda(TyLambda),
    TyApp(TyApp),
    Pack(Pack),
    Unpack(Unpack),
    Record(Record),
    RecordProj(RecordProj),
}

impl SubstTerm for Term {
    fn subst(self, v: &Var, t: Term) -> Term {
        match self {
            Term::Var(var) => {
                if var == *v {
                    t
                } else {
                    Term::Var(var)
                }
            }
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::TyLambda(lam) => lam.subst(v, t),
            Term::TyApp(app) => app.subst(v, t),
            Term::Pack(pack) => pack.subst(v, t),
            Term::Unpack(unpack) => unpack.subst(v, t),
            Term::Record(rec) => rec.subst(v, t),
            Term::RecordProj(proj) => proj.subst(v, t),
        }
    }
}

impl SubstTy for Term {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        match self {
            Term::Var(v) => Term::Var(v),
            Term::Lambda(lam) => lam.subst_ty(v, ty).into(),
            Term::App(app) => app.subst_ty(v, ty).into(),
            Term::TyLambda(lam) => lam.subst_ty(v, ty).into(),
            Term::TyApp(app) => app.subst_ty(v, ty).into(),
            Term::Pack(pack) => pack.subst_ty(v, ty).into(),
            Term::Unpack(unpack) => unpack.subst_ty(v, ty).into(),
            Term::Record(rec) => rec.subst_ty(v, ty).into(),
            Term::RecordProj(proj) => proj.subst_ty(v, ty).into(),
        }
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
            Term::TyLambda(abs) => abs.fmt(f),
            Term::TyApp(app) => app.fmt(f),
            Term::Pack(pack) => pack.fmt(f),
            Term::Unpack(unpack) => unpack.fmt(f),
            Term::Record(rec) => rec.fmt(f),
            Term::RecordProj(proj) => proj.fmt(f),
        }
    }
}
