use super::SubstTy;
use crate::{
    terms::Term,
    types::{Type, TypeVar},
};

impl SubstTy for Term {
    fn subst_ty(self, v: &TypeVar, ty: Type) -> Self {
        match self {
            Term::Var(var) => Term::Var(var),
            Term::Unit => Term::Unit,
            Term::Lambda(lam) => lam.subst_ty(v, ty).into(),
            Term::App(app) => app.subst_ty(v, ty).into(),
            Term::Pack(pack) => pack.subst_ty(v, ty).into(),
            Term::Unpack(unpack) => unpack.subst_ty(v, ty).into(),
            Term::Record(rec) => rec.subst_ty(v, ty).into(),
            Term::RecordProj(proj) => proj.subst_ty(v, ty).into(),
            Term::Zero(zero) => zero.subst_ty(v, ty).into(),
            Term::Succ(succ) => succ.subst_ty(v, ty).into(),
            Term::Pred(pred) => pred.subst_ty(v, ty).into(),
            Term::IsZero(isz) => isz.subst_ty(v, ty).into(),
            Term::True(tru) => tru.subst_ty(v, ty).into(),
            Term::False(fls) => fls.subst_ty(v, ty).into(),
            Term::If(ift) => ift.subst_ty(v, ty).into(),
            Term::Fix(fix) => fix.subst_ty(v, ty).into(),
        }
    }
}
