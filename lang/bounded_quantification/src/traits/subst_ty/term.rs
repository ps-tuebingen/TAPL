use super::SubstTy;
use crate::{
    syntax::Term,
    types::{Type, TypeVar},
};

impl SubstTy for Term {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        match self {
            Term::Var(_) => self,
            Term::Const(c) => c.subst_ty(v, ty).into(),
            Term::Succ(s) => s.subst_ty(v, ty).into(),
            Term::Pred(p) => p.subst_ty(v, ty).into(),
            Term::App(app) => app.subst_ty(v, ty).into(),
            Term::Lambda(lam) => lam.subst_ty(v, ty).into(),
            Term::LambdaSub(lam) => lam.subst_ty(v, ty).into(),
            Term::TyApp(app) => app.subst_ty(v, ty).into(),
            Term::Pack(pack) => pack.subst_ty(v, ty).into(),
            Term::Unpack(unpack) => unpack.subst_ty(v, ty).into(),
        }
    }
}
