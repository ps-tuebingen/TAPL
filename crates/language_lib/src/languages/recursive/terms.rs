use super::types::Type;
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
    Var(Variable<Term>),
    Lambda(Lambda<Term, Type>),
    App(App<Term>),
    Unit(Unit<Term>),
    Fold(Fold<Term, Type>),
    Unfold(Unfold<Term, Type>),
    Variant(Variant<Term, Type>),
    VariantCase(VariantCase<Term>),
    Pair(Pair<Term>),
    Fst(Fst<Term>),
    Snd(Snd<Term>),
    Num(Num<Term>),
    Succ(Succ<Term>),
    Pred(Pred<Term>),
    IsZero(IsZero<Term>),
    True(True<Term>),
    False(False<Term>),
    If(If<Term>),
    Fix(Fix<Term>),
    Let(Let<Term>),
    Record(Record<Term>),
    RecordProj(RecordProj<Term>),
}

impl syntax::terms::Term for Term {}

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            Variable::<Term>::rule(),
            Lambda::<Term, Type>::rule(),
            App::<Term>::rule(),
            Unit::<Term>::rule(),
            Fold::<Term, Type>::rule(),
            Unfold::<Term, Type>::rule(),
            Variant::<Term, Type>::rule(),
            VariantCase::<Term>::rule(),
            Pair::<Term>::rule(),
            Fst::<Term>::rule(),
            Snd::<Term>::rule(),
            Num::<Term>::rule(),
            Succ::<Term>::rule(),
            Pred::<Term>::rule(),
            IsZero::<Term>::rule(),
            True::<Term>::rule(),
            False::<Term>::rule(),
            If::<Term>::rule(),
            Fix::<Term>::rule(),
            Let::<Term>::rule(),
            Record::<Term>::rule(),
            RecordProj::<Term>::rule(),
        ])
    }
}

impl SubstType<Type> for Term {
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Term::Var(var) => var.subst_type(v, ty),
            Term::Lambda(lam) => lam.subst_type(v, ty),
            Term::App(app) => app.subst_type(v, ty),
            Term::Unit(u) => u.subst_type(v, ty),
            Term::Fold(fold) => fold.subst_type(v, ty),
            Term::Unfold(unfold) => unfold.subst_type(v, ty),
            Term::Variant(var) => var.subst_type(v, ty),
            Term::VariantCase(case) => case.subst_type(v, ty),
            Term::Pair(p) => p.subst_type(v, ty),
            Term::Fst(fst) => fst.subst_type(v, ty),
            Term::Snd(snd) => snd.subst_type(v, ty),
            Term::Num(num) => num.subst_type(v, ty),
            Term::Succ(succ) => succ.subst_type(v, ty),
            Term::Pred(pred) => pred.subst_type(v, ty),
            Term::IsZero(isz) => isz.subst_type(v, ty),
            Term::True(tru) => tru.subst_type(v, ty),
            Term::False(fls) => fls.subst_type(v, ty),
            Term::If(ift) => ift.subst_type(v, ty),
            Term::Fix(fix) => fix.subst_type(v, ty),
            Term::Let(lt) => lt.subst_type(v, ty),
            Term::Record(rec) => rec.subst_type(v, ty),
            Term::RecordProj(proj) => proj.subst_type(v, ty),
        }
    }
}

impl SubstTerm<Term> for Term {
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::Unit(u) => u.subst(v, t),
            Term::Fold(fold) => fold.subst(v, t),
            Term::Unfold(unfold) => unfold.subst(v, t),
            Term::Variant(var) => var.subst(v, t),
            Term::VariantCase(case) => case.subst(v, t),
            Term::Pair(p) => p.subst(v, t),
            Term::Fst(fst) => fst.subst(v, t),
            Term::Snd(snd) => snd.subst(v, t),
            Term::Num(num) => num.subst(v, t),
            Term::Succ(succ) => succ.subst(v, t),
            Term::Pred(pred) => pred.subst(v, t),
            Term::IsZero(isz) => isz.subst(v, t),
            Term::True(tru) => tru.subst(v, t),
            Term::False(fls) => fls.subst(v, t),
            Term::If(ift) => ift.subst(v, t),
            Term::Fix(fix) => fix.subst(v, t),
            Term::Let(lt) => lt.subst(v, t),
            Term::Record(rec) => rec.subst(v, t),
            Term::RecordProj(proj) => proj.subst(v, t),
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

impl From<Fold<Term, Type>> for Term {
    fn from(fld: Fold<Term, Type>) -> Term {
        Term::Fold(fld)
    }
}

impl From<Unfold<Term, Type>> for Term {
    fn from(unfld: Unfold<Term, Type>) -> Term {
        Term::Unfold(unfld)
    }
}
impl From<Lambda<Term, Type>> for Term {
    fn from(lam: Lambda<Term, Type>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<Unit<Term>> for Term {
    fn from(u: Unit<Term>) -> Term {
        Term::Unit(u)
    }
}

impl From<True<Term>> for Term {
    fn from(tru: True<Term>) -> Term {
        Term::True(tru)
    }
}

impl From<False<Term>> for Term {
    fn from(fls: False<Term>) -> Term {
        Term::False(fls)
    }
}

impl From<Num<Term>> for Term {
    fn from(num: Num<Term>) -> Term {
        Term::Num(num)
    }
}

impl From<Pair<Term>> for Term {
    fn from(pair: Pair<Term>) -> Term {
        Term::Pair(pair)
    }
}

impl From<Record<Term>> for Term {
    fn from(rec: Record<Term>) -> Term {
        Term::Record(rec)
    }
}

impl From<Variant<Term, Type>> for Term {
    fn from(var: Variant<Term, Type>) -> Term {
        Term::Variant(var)
    }
}
impl From<Variable<Term>> for Term {
    fn from(var: Variable<Term>) -> Term {
        Term::Var(var)
    }
}

impl From<App<Term>> for Term {
    fn from(app: App<Term>) -> Term {
        Term::App(app)
    }
}

impl From<If<Term>> for Term {
    fn from(ift: If<Term>) -> Term {
        Term::If(ift)
    }
}

impl From<Pred<Term>> for Term {
    fn from(pred: Pred<Term>) -> Term {
        Term::Pred(pred)
    }
}

impl From<Succ<Term>> for Term {
    fn from(succ: Succ<Term>) -> Term {
        Term::Succ(succ)
    }
}

impl From<IsZero<Term>> for Term {
    fn from(isz: IsZero<Term>) -> Term {
        Term::IsZero(isz)
    }
}

impl From<Let<Term>> for Term {
    fn from(lt: Let<Term>) -> Term {
        Term::Let(lt)
    }
}

impl From<Fst<Term>> for Term {
    fn from(fst: Fst<Term>) -> Term {
        Term::Fst(fst)
    }
}

impl From<Snd<Term>> for Term {
    fn from(snd: Snd<Term>) -> Term {
        Term::Snd(snd)
    }
}

impl From<RecordProj<Term>> for Term {
    fn from(proj: RecordProj<Term>) -> Term {
        Term::RecordProj(proj)
    }
}

impl From<VariantCase<Term>> for Term {
    fn from(case: VariantCase<Term>) -> Term {
        Term::VariantCase(case)
    }
}

impl From<Fix<Term>> for Term {
    fn from(fix: Fix<Term>) -> Term {
        Term::Fix(fix)
    }
}
