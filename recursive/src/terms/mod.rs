use crate::{
    errors::ErrorKind,
    traits::{
        is_value::IsValue,
        subst::{SubstTerm, SubstTy},
    },
    types::{Type, TypeVar},
};
use std::fmt;

pub mod bool;
pub mod fix;
pub mod fold;
pub mod lambda;
pub mod let_exp;
pub mod nat;
pub mod pair;
pub mod record;
pub mod variant;

pub use bool::{False, If, True};
pub use fix::Fix;
pub use fold::{Fold, Unfold};
pub use lambda::{App, Lambda};
pub use let_exp::Let;
pub use nat::{IsZero, Pred, Succ, Zero};
pub use pair::{Fst, Pair, Snd};
pub use record::{Record, RecordProj};
pub use variant::{Variant, VariantCase, VariantPattern};

pub type Var = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Var),
    Lambda(Lambda),
    App(App),
    Unit,
    Fold(Fold),
    Unfold(Unfold),
    Variant(Variant),
    VariantCase(VariantCase),
    Pair(Pair),
    Fst(Fst),
    Snd(Snd),
    Zero(Zero),
    Succ(Succ),
    Pred(Pred),
    IsZero(IsZero),
    True(True),
    False(False),
    If(If),
    Fix(Fix),
    Let(Let),
    Record(Record),
    RecordProj(RecordProj),
}

impl Term {
    pub fn as_var(&self) -> Result<Var, ErrorKind> {
        if let Term::Var(v) = self {
            Ok(v.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Variable"))
        }
    }

    pub fn as_lambda(&self) -> Result<Lambda, ErrorKind> {
        if let Term::Lambda(lam) = self {
            Ok(lam.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Lambda Abstraction"))
        }
    }

    pub fn as_app(&self) -> Result<App, ErrorKind> {
        if let Term::App(app) = self {
            Ok(app.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Application"))
        }
    }

    pub fn as_unit(&self) -> Result<(), ErrorKind> {
        if let Term::Unit = self {
            Ok(())
        } else {
            Err(ErrorKind::unexpected_term(self, "Unit"))
        }
    }

    pub fn as_fold(&self) -> Result<Fold, ErrorKind> {
        if let Term::Fold(fold) = self {
            Ok(fold.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Fold"))
        }
    }

    pub fn as_unfold(&self) -> Result<Unfold, ErrorKind> {
        if let Term::Unfold(unfold) = self {
            Ok(unfold.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Unfold"))
        }
    }

    pub fn as_variant(&self) -> Result<Variant, ErrorKind> {
        if let Term::Variant(var) = self {
            Ok(var.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Variant"))
        }
    }

    pub fn as_variantcase(&self) -> Result<VariantCase, ErrorKind> {
        if let Term::VariantCase(case) = self {
            Ok(case.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Variant Case"))
        }
    }
    pub fn as_pair(&self) -> Result<Pair, ErrorKind> {
        if let Term::Pair(pair) = self {
            Ok(pair.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Pair"))
        }
    }

    pub fn as_fst(&self) -> Result<Fst, ErrorKind> {
        if let Term::Fst(fst) = self {
            Ok(fst.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Fst"))
        }
    }

    pub fn as_snd(&self) -> Result<Snd, ErrorKind> {
        if let Term::Snd(snd) = self {
            Ok(snd.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Snd"))
        }
    }

    pub fn as_zero(&self) -> Result<Zero, ErrorKind> {
        if let Term::Zero(zero) = self {
            Ok(zero.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Zero"))
        }
    }

    pub fn as_succ(&self) -> Result<Succ, ErrorKind> {
        if let Term::Succ(succ) = self {
            Ok(succ.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Succ"))
        }
    }

    pub fn as_pred(&self) -> Result<Pred, ErrorKind> {
        if let Term::Pred(pred) = self {
            Ok(pred.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Pred"))
        }
    }

    pub fn as_iszero(&self) -> Result<IsZero, ErrorKind> {
        if let Term::IsZero(isz) = self {
            Ok(isz.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "IsZero"))
        }
    }

    pub fn as_true(&self) -> Result<True, ErrorKind> {
        if let Term::True(tru) = self {
            Ok(tru.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "True"))
        }
    }

    pub fn as_false(&self) -> Result<False, ErrorKind> {
        if let Term::False(fls) = self {
            Ok(fls.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "False"))
        }
    }

    pub fn as_if(&self) -> Result<If, ErrorKind> {
        if let Term::If(ift) = self {
            Ok(ift.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "If"))
        }
    }

    pub fn as_fix(&self) -> Result<Fix, ErrorKind> {
        if let Term::Fix(fix) = self {
            Ok(fix.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Fix"))
        }
    }

    pub fn as_let(&self) -> Result<Let, ErrorKind> {
        if let Term::Let(lt) = self {
            Ok(lt.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Let"))
        }
    }

    pub fn as_record(&self) -> Result<Record, ErrorKind> {
        if let Term::Record(rec) = self {
            Ok(rec.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Record"))
        }
    }

    pub fn as_recordproj(&self) -> Result<RecordProj, ErrorKind> {
        if let Term::RecordProj(proj) = self {
            Ok(proj.clone())
        } else {
            Err(ErrorKind::unexpected_term(self, "Record Projection"))
        }
    }
}

impl IsValue for Term {
    fn is_value(&self) -> bool {
        match self {
            Term::Var(_) => false,
            Term::Lambda(lam) => lam.is_value(),
            Term::App(app) => app.is_value(),
            Term::Unit => false,
            Term::Fold(fold) => fold.is_value(),
            Term::Unfold(unfold) => unfold.is_value(),
            Term::Variant(var) => var.is_value(),
            Term::VariantCase(case) => case.is_value(),
            Term::Pair(p) => p.is_value(),
            Term::Fst(fst) => fst.is_value(),
            Term::Snd(snd) => snd.is_value(),
            Term::Zero(zero) => zero.is_value(),
            Term::Succ(succ) => succ.is_value(),
            Term::Pred(pred) => pred.is_value(),
            Term::IsZero(isz) => isz.is_value(),
            Term::True(tru) => tru.is_value(),
            Term::False(fls) => fls.is_value(),
            Term::If(ift) => ift.is_value(),
            Term::Fix(fix) => fix.is_value(),
            Term::Let(lt) => lt.is_value(),
            Term::Record(rec) => rec.is_value(),
            Term::RecordProj(proj) => proj.is_value(),
        }
    }
}

impl SubstTy for Term {
    fn subst_ty(self, v: TypeVar, ty: Type) -> Self {
        match self {
            Term::Var(_) => self,
            Term::Lambda(lam) => lam.subst_ty(v, ty).into(),
            Term::App(app) => app.subst_ty(v, ty).into(),
            Term::Unit => self,
            Term::Fold(fold) => fold.subst_ty(v, ty).into(),
            Term::Unfold(unfold) => unfold.subst_ty(v, ty).into(),
            Term::Variant(var) => var.subst_ty(v, ty).into(),
            Term::VariantCase(case) => case.subst_ty(v, ty).into(),
            Term::Pair(p) => p.subst_ty(v, ty).into(),
            Term::Fst(fst) => fst.subst_ty(v, ty).into(),
            Term::Snd(snd) => snd.subst_ty(v, ty).into(),
            Term::Zero(zero) => zero.subst_ty(v, ty).into(),
            Term::Succ(succ) => succ.subst_ty(v, ty).into(),
            Term::Pred(pred) => pred.subst_ty(v, ty).into(),
            Term::IsZero(isz) => isz.subst_ty(v, ty).into(),
            Term::True(tru) => tru.subst_ty(v, ty).into(),
            Term::False(fls) => fls.subst_ty(v, ty).into(),
            Term::If(ift) => ift.subst_ty(v, ty).into(),
            Term::Fix(fix) => fix.subst_ty(v, ty).into(),
            Term::Let(lt) => lt.subst_ty(v, ty).into(),
            Term::Record(rec) => rec.subst_ty(v, ty).into(),
            Term::RecordProj(proj) => proj.subst_ty(v, ty).into(),
        }
    }
}

impl SubstTerm for Term {
    fn subst(self, v: Var, t: Term) -> Term {
        match self {
            Term::Var(_) => self,
            Term::Lambda(lam) => lam.subst(v, t).into(),
            Term::App(app) => app.subst(v, t).into(),
            Term::Unit => self,
            Term::Fold(fold) => fold.subst(v, t).into(),
            Term::Unfold(unfold) => unfold.subst(v, t).into(),
            Term::Variant(var) => var.subst(v, t).into(),
            Term::VariantCase(case) => case.subst(v, t).into(),
            Term::Pair(p) => p.subst(v, t).into(),
            Term::Fst(fst) => fst.subst(v, t).into(),
            Term::Snd(snd) => snd.subst(v, t).into(),
            Term::Zero(zero) => zero.subst(v, t).into(),
            Term::Succ(succ) => succ.subst(v, t).into(),
            Term::Pred(pred) => pred.subst(v, t).into(),
            Term::IsZero(isz) => isz.subst(v, t).into(),
            Term::True(tru) => tru.subst(v, t).into(),
            Term::False(fls) => fls.subst(v, t).into(),
            Term::If(ift) => ift.subst(v, t).into(),
            Term::Fix(fix) => fix.subst(v, t).into(),
            Term::Let(lt) => lt.subst(v, t).into(),
            Term::Record(rec) => rec.subst(v, t).into(),
            Term::RecordProj(proj) => proj.subst(v, t).into(),
        }
    }
}

impl From<&str> for Term {
    fn from(s: &str) -> Term {
        Term::Var(s.to_owned())
    }
}

impl From<String> for Term {
    fn from(s: String) -> Term {
        Term::Var(s)
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => f.write_str(v),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::Unit => f.write_str("unit"),
            Term::Fold(fold) => fold.fmt(f),
            Term::Unfold(unfold) => unfold.fmt(f),
            Term::Variant(var) => var.fmt(f),
            Term::VariantCase(case) => case.fmt(f),
            Term::Pair(p) => p.fmt(f),
            Term::Fst(fst) => fst.fmt(f),
            Term::Snd(snd) => snd.fmt(f),
            Term::Zero(zero) => zero.fmt(f),
            Term::Succ(succ) => succ.fmt(f),
            Term::Pred(pred) => pred.fmt(f),
            Term::IsZero(isz) => isz.fmt(f),
            Term::True(tru) => tru.fmt(f),
            Term::False(fls) => fls.fmt(f),
            Term::If(ift) => ift.fmt(f),
            Term::Fix(fix) => fix.fmt(f),
            Term::Let(lt) => lt.fmt(f),
            Term::Record(rec) => rec.fmt(f),
            Term::RecordProj(proj) => proj.fmt(f),
        }
    }
}
