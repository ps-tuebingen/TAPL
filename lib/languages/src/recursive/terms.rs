use super::{Recursive, types::Type};
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{
        App, False, Fix, Fold, Fst, If, IsZero, Lambda, Let, Num, Pair, Pred, Record, RecordProj,
        Snd, Succ, True, Unfold, Unit, Variable, Variant, VariantCase,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<Recursive>),
    Lambda(Lambda<Recursive>),
    App(App<Recursive>),
    Unit(Unit<Recursive>),
    Fold(Fold<Recursive>),
    Unfold(Unfold<Recursive>),
    Variant(Variant<Recursive>),
    VariantCase(VariantCase<Recursive>),
    Pair(Pair<Recursive>),
    Fst(Fst<Recursive>),
    Snd(Snd<Recursive>),
    Num(Num<Recursive>),
    Succ(Succ<Recursive>),
    Pred(Pred<Recursive>),
    IsZero(IsZero<Recursive>),
    True(True<Recursive>),
    False(False<Recursive>),
    If(If<Recursive>),
    Fix(Fix<Recursive>),
    Let(Let<Recursive>),
    Record(Record<Recursive>),
    RecordProj(RecordProj<Recursive>),
}

impl syntax::terms::Term for Term {}

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            Variable::<Recursive>::rule(),
            Lambda::<Recursive>::rule(),
            App::<Recursive>::rule(),
            Unit::<Recursive>::rule(),
            Fold::<Recursive>::rule(),
            Unfold::<Recursive>::rule(),
            Variant::<Recursive>::rule(),
            VariantCase::<Recursive>::rule(),
            Pair::<Recursive>::rule(),
            Fst::<Recursive>::rule(),
            Snd::<Recursive>::rule(),
            Num::<Recursive>::rule(),
            Succ::<Recursive>::rule(),
            Pred::<Recursive>::rule(),
            IsZero::<Recursive>::rule(),
            True::<Recursive>::rule(),
            False::<Recursive>::rule(),
            If::<Recursive>::rule(),
            Fix::<Recursive>::rule(),
            Let::<Recursive>::rule(),
            Record::<Recursive>::rule(),
            RecordProj::<Recursive>::rule(),
        ])
    }
}

impl SubstType for Term {
    type Lang = Recursive;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Term::Var(var) => var.subst_type(v, ty).into(),
            Term::Lambda(lam) => lam.subst_type(v, ty).into(),
            Term::App(app) => app.subst_type(v, ty).into(),
            Term::Unit(u) => u.subst_type(v, ty).into(),
            Term::Fold(fold) => fold.subst_type(v, ty).into(),
            Term::Unfold(unfold) => unfold.subst_type(v, ty).into(),
            Term::Variant(var) => var.subst_type(v, ty).into(),
            Term::VariantCase(case) => case.subst_type(v, ty).into(),
            Term::Pair(p) => p.subst_type(v, ty).into(),
            Term::Fst(fst) => fst.subst_type(v, ty).into(),
            Term::Snd(snd) => snd.subst_type(v, ty).into(),
            Term::Num(num) => num.subst_type(v, ty).into(),
            Term::Succ(succ) => succ.subst_type(v, ty).into(),
            Term::Pred(pred) => pred.subst_type(v, ty).into(),
            Term::IsZero(isz) => isz.subst_type(v, ty).into(),
            Term::True(tru) => tru.subst_type(v, ty).into(),
            Term::False(fls) => fls.subst_type(v, ty).into(),
            Term::If(ift) => ift.subst_type(v, ty).into(),
            Term::Fix(fix) => fix.subst_type(v, ty).into(),
            Term::Let(lt) => lt.subst_type(v, ty).into(),
            Term::Record(rec) => rec.subst_type(v, ty).into(),
            Term::RecordProj(proj) => proj.subst_type(v, ty).into(),
        }
    }
}

impl SubstTerm for Term {
    type Lang = Recursive;
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t).into(),
            Term::Lambda(lam) => lam.subst(v, t).into(),
            Term::App(app) => app.subst(v, t).into(),
            Term::Unit(u) => u.subst(v, t).into(),
            Term::Fold(fold) => fold.subst(v, t).into(),
            Term::Unfold(unfold) => unfold.subst(v, t).into(),
            Term::Variant(var) => var.subst(v, t).into(),
            Term::VariantCase(case) => case.subst(v, t).into(),
            Term::Pair(p) => p.subst(v, t).into(),
            Term::Fst(fst) => fst.subst(v, t).into(),
            Term::Snd(snd) => snd.subst(v, t).into(),
            Term::Num(num) => num.subst(v, t).into(),
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

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(v) => v.fmt(f),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::Unit(u) => u.fmt(f),
            Term::Fold(fold) => fold.fmt(f),
            Term::Unfold(unfold) => unfold.fmt(f),
            Term::Variant(var) => var.fmt(f),
            Term::VariantCase(case) => case.fmt(f),
            Term::Pair(p) => p.fmt(f),
            Term::Fst(fst) => fst.fmt(f),
            Term::Snd(snd) => snd.fmt(f),
            Term::Num(num) => num.fmt(f),
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

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Var(v) => v.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::Unit(u) => u.to_latex(conf),
            Term::Fold(fold) => fold.to_latex(conf),
            Term::Unfold(unfold) => unfold.to_latex(conf),
            Term::Variant(var) => var.to_latex(conf),
            Term::VariantCase(case) => case.to_latex(conf),
            Term::Pair(p) => p.to_latex(conf),
            Term::Fst(fst) => fst.to_latex(conf),
            Term::Snd(snd) => snd.to_latex(conf),
            Term::Num(num) => num.to_latex(conf),
            Term::Succ(succ) => succ.to_latex(conf),
            Term::Pred(pred) => pred.to_latex(conf),
            Term::IsZero(isz) => isz.to_latex(conf),
            Term::True(tru) => tru.to_latex(conf),
            Term::False(fls) => fls.to_latex(conf),
            Term::If(ift) => ift.to_latex(conf),
            Term::Fix(fix) => fix.to_latex(conf),
            Term::Let(lt) => lt.to_latex(conf),
            Term::Record(rec) => rec.to_latex(conf),
            Term::RecordProj(proj) => proj.to_latex(conf),
        }
    }
}

impl From<Fold<Recursive>> for Term {
    fn from(fld: Fold<Recursive>) -> Term {
        Term::Fold(fld)
    }
}

impl From<Unfold<Recursive>> for Term {
    fn from(unfld: Unfold<Recursive>) -> Term {
        Term::Unfold(unfld)
    }
}
impl From<Lambda<Recursive>> for Term {
    fn from(lam: Lambda<Recursive>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<Unit<Recursive>> for Term {
    fn from(u: Unit<Recursive>) -> Term {
        Term::Unit(u)
    }
}

impl From<True<Recursive>> for Term {
    fn from(tru: True<Recursive>) -> Term {
        Term::True(tru)
    }
}

impl From<False<Recursive>> for Term {
    fn from(fls: False<Recursive>) -> Term {
        Term::False(fls)
    }
}

impl From<Num<Recursive>> for Term {
    fn from(num: Num<Recursive>) -> Term {
        Term::Num(num)
    }
}

impl From<Pair<Recursive>> for Term {
    fn from(pair: Pair<Recursive>) -> Term {
        Term::Pair(pair)
    }
}

impl From<Record<Recursive>> for Term {
    fn from(rec: Record<Recursive>) -> Term {
        Term::Record(rec)
    }
}

impl From<Variant<Recursive>> for Term {
    fn from(var: Variant<Recursive>) -> Term {
        Term::Variant(var)
    }
}
impl From<Variable<Recursive>> for Term {
    fn from(var: Variable<Recursive>) -> Term {
        Term::Var(var)
    }
}

impl From<App<Recursive>> for Term {
    fn from(app: App<Recursive>) -> Term {
        Term::App(app)
    }
}

impl From<If<Recursive>> for Term {
    fn from(ift: If<Recursive>) -> Term {
        Term::If(ift)
    }
}

impl From<Pred<Recursive>> for Term {
    fn from(pred: Pred<Recursive>) -> Term {
        Term::Pred(pred)
    }
}

impl From<Succ<Recursive>> for Term {
    fn from(succ: Succ<Recursive>) -> Term {
        Term::Succ(succ)
    }
}

impl From<IsZero<Recursive>> for Term {
    fn from(isz: IsZero<Recursive>) -> Term {
        Term::IsZero(isz)
    }
}

impl From<Let<Recursive>> for Term {
    fn from(lt: Let<Recursive>) -> Term {
        Term::Let(lt)
    }
}

impl From<Fst<Recursive>> for Term {
    fn from(fst: Fst<Recursive>) -> Term {
        Term::Fst(fst)
    }
}

impl From<Snd<Recursive>> for Term {
    fn from(snd: Snd<Recursive>) -> Term {
        Term::Snd(snd)
    }
}

impl From<RecordProj<Recursive>> for Term {
    fn from(proj: RecordProj<Recursive>) -> Term {
        Term::RecordProj(proj)
    }
}

impl From<VariantCase<Recursive>> for Term {
    fn from(case: VariantCase<Recursive>) -> Term {
        Term::VariantCase(case)
    }
}

impl From<Fix<Recursive>> for Term {
    fn from(fix: Fix<Recursive>) -> Term {
        Term::Fix(fix)
    }
}
