use super::{Subtypes, types::Type};
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{
        App, Assign, Cast, Cons, Deref, False, Fix, If, Lambda, Let, ListCase, Loc, Nil, Num, Pred,
        Record, RecordProj, Ref, Succ, True, Unit, Variable, Variant, VariantCase,
    },
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Term {
    Var(Variable<Subtypes>),
    Lambda(Lambda<Subtypes>),
    App(App<Subtypes>),
    Unit(Unit<Subtypes>),
    Cast(Cast<Subtypes>),
    Record(Record<Subtypes>),
    RecordProj(RecordProj<Subtypes>),
    Variant(Variant<Subtypes>),
    VariantCase(VariantCase<Subtypes>),
    Nil(Nil<Subtypes>),
    Cons(Cons<Subtypes>),
    ListCase(ListCase<Subtypes>),
    Ref(Ref<Subtypes>),
    Deref(Deref<Subtypes>),
    Assign(Assign<Subtypes>),
    Loc(Loc<Subtypes>),
    Num(Num<Subtypes>),
    Succ(Succ<Subtypes>),
    Pred(Pred<Subtypes>),
    True(True<Subtypes>),
    False(False<Subtypes>),
    If(If<Subtypes>),
    Let(Let<Subtypes>),
    Fix(Fix<Subtypes>),
}

impl syntax::terms::Term for Term {}

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            Variable::<Subtypes>::rule(),
            Lambda::<Subtypes>::rule(),
            App::<Subtypes>::rule(),
            Unit::<Subtypes>::rule(),
            Record::<Subtypes>::rule(),
            RecordProj::<Subtypes>::rule(),
            Variant::<Subtypes>::rule(),
            VariantCase::<Subtypes>::rule(),
            Cast::<Subtypes>::rule(),
            Nil::<Subtypes>::rule(),
            Cons::<Subtypes>::rule(),
            ListCase::<Subtypes>::rule(),
            Ref::<Subtypes>::rule(),
            Deref::<Subtypes>::rule(),
            Assign::<Subtypes>::rule(),
            Loc::<Subtypes>::rule(),
            Num::<Subtypes>::rule(),
            Succ::<Subtypes>::rule(),
            Pred::<Subtypes>::rule(),
            True::<Subtypes>::rule(),
            False::<Subtypes>::rule(),
            If::<Subtypes>::rule(),
            Let::<Subtypes>::rule(),
            Fix::<Subtypes>::rule(),
        ])
    }
}

impl SubstType for Term {
    type Lang = Subtypes;
    type Target = Term;
    fn subst_type(self, _: &TypeVar, _: &Type) -> Self::Target {
        self
    }
}

impl SubstTerm for Term {
    type Lang = Subtypes;
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t).into(),
            Term::Lambda(lam) => lam.subst(v, t).into(),
            Term::App(app) => app.subst(v, t).into(),
            Term::Unit(unit) => unit.subst(v, t).into(),
            Term::Record(rec) => rec.subst(v, t).into(),
            Term::RecordProj(proj) => proj.subst(v, t).into(),
            Term::Variant(var) => var.subst(v, t).into(),
            Term::VariantCase(case) => case.subst(v, t).into(),
            Term::Cast(cast) => cast.subst(v, t).into(),
            Term::Nil(nil) => nil.subst(v, t).into(),
            Term::Cons(cons) => cons.subst(v, t).into(),
            Term::ListCase(case) => case.subst(v, t).into(),
            Term::Ref(rf) => rf.subst(v, t).into(),
            Term::Deref(deref) => deref.subst(v, t).into(),
            Term::Assign(assign) => assign.subst(v, t).into(),
            Term::Loc(loc) => loc.subst(v, t).into(),
            Term::Num(num) => num.subst(v, t).into(),
            Term::Succ(succ) => succ.subst(v, t).into(),
            Term::Pred(pred) => pred.subst(v, t).into(),
            Term::True(tru) => tru.subst(v, t).into(),
            Term::False(fls) => fls.subst(v, t).into(),
            Term::If(ift) => ift.subst(v, t).into(),
            Term::Let(lt) => lt.subst(v, t).into(),
            Term::Fix(fix) => fix.subst(v, t).into(),
        }
    }
}

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Var(var) => var.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::Unit(unit) => unit.to_latex(conf),
            Term::Record(rec) => rec.to_latex(conf),
            Term::RecordProj(proj) => proj.to_latex(conf),
            Term::Variant(var) => var.to_latex(conf),
            Term::VariantCase(case) => case.to_latex(conf),
            Term::Cast(cast) => cast.to_latex(conf),
            Term::Nil(nil) => nil.to_latex(conf),
            Term::Cons(cons) => cons.to_latex(conf),
            Term::ListCase(case) => case.to_latex(conf),
            Term::Ref(rf) => rf.to_latex(conf),
            Term::Deref(deref) => deref.to_latex(conf),
            Term::Assign(assign) => assign.to_latex(conf),
            Term::Loc(loc) => loc.to_latex(conf),
            Term::Num(num) => num.to_latex(conf),
            Term::Succ(succ) => succ.to_latex(conf),
            Term::Pred(pred) => pred.to_latex(conf),
            Term::True(tru) => tru.to_latex(conf),
            Term::False(fls) => fls.to_latex(conf),
            Term::If(ift) => ift.to_latex(conf),
            Term::Let(lt) => lt.to_latex(conf),
            Term::Fix(fix) => fix.to_latex(conf),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(var) => var.fmt(f),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::Unit(unit) => unit.fmt(f),
            Term::Record(rec) => rec.fmt(f),
            Term::RecordProj(proj) => proj.fmt(f),
            Term::Variant(var) => var.fmt(f),
            Term::VariantCase(case) => case.fmt(f),
            Term::Cast(cast) => cast.fmt(f),
            Term::Nil(nil) => nil.fmt(f),
            Term::Cons(cons) => cons.fmt(f),
            Term::ListCase(case) => case.fmt(f),
            Term::Ref(rf) => rf.fmt(f),
            Term::Deref(deref) => deref.fmt(f),
            Term::Assign(assign) => assign.fmt(f),
            Term::Loc(loc) => loc.fmt(f),
            Term::Num(num) => num.fmt(f),
            Term::Succ(succ) => succ.fmt(f),
            Term::Pred(pred) => pred.fmt(f),
            Term::True(tru) => tru.fmt(f),
            Term::False(fls) => fls.fmt(f),
            Term::If(ift) => ift.fmt(f),
            Term::Let(lt) => lt.fmt(f),
            Term::Fix(fix) => fix.fmt(f),
        }
    }
}

impl From<Loc<Subtypes>> for Term {
    fn from(loc: Loc<Subtypes>) -> Term {
        Term::Loc(loc)
    }
}
impl From<Assign<Subtypes>> for Term {
    fn from(assign: Assign<Subtypes>) -> Term {
        Term::Assign(assign)
    }
}
impl From<Deref<Subtypes>> for Term {
    fn from(deref: Deref<Subtypes>) -> Term {
        Term::Deref(deref)
    }
}
impl From<Ref<Subtypes>> for Term {
    fn from(reft: Ref<Subtypes>) -> Term {
        Term::Ref(reft)
    }
}
impl From<ListCase<Subtypes>> for Term {
    fn from(case: ListCase<Subtypes>) -> Term {
        Term::ListCase(case)
    }
}

impl From<Cast<Subtypes>> for Term {
    fn from(cast: Cast<Subtypes>) -> Term {
        Term::Cast(cast)
    }
}

impl From<Lambda<Subtypes>> for Term {
    fn from(lam: Lambda<Subtypes>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<Unit<Subtypes>> for Term {
    fn from(u: Unit<Subtypes>) -> Term {
        Term::Unit(u)
    }
}

impl From<True<Subtypes>> for Term {
    fn from(tru: True<Subtypes>) -> Term {
        Term::True(tru)
    }
}

impl From<False<Subtypes>> for Term {
    fn from(fls: False<Subtypes>) -> Term {
        Term::False(fls)
    }
}

impl From<Num<Subtypes>> for Term {
    fn from(num: Num<Subtypes>) -> Term {
        Term::Num(num)
    }
}

impl From<Record<Subtypes>> for Term {
    fn from(rec: Record<Subtypes>) -> Term {
        Term::Record(rec)
    }
}

impl From<Variant<Subtypes>> for Term {
    fn from(var: Variant<Subtypes>) -> Term {
        Term::Variant(var)
    }
}

impl From<Nil<Subtypes>> for Term {
    fn from(nil: Nil<Subtypes>) -> Term {
        Term::Nil(nil)
    }
}

impl From<Cons<Subtypes>> for Term {
    fn from(cons: Cons<Subtypes>) -> Term {
        Term::Cons(cons)
    }
}

impl From<Variable<Subtypes>> for Term {
    fn from(var: Variable<Subtypes>) -> Term {
        Term::Var(var)
    }
}

impl From<App<Subtypes>> for Term {
    fn from(app: App<Subtypes>) -> Term {
        Term::App(app)
    }
}

impl From<If<Subtypes>> for Term {
    fn from(ift: If<Subtypes>) -> Term {
        Term::If(ift)
    }
}

impl From<Pred<Subtypes>> for Term {
    fn from(pred: Pred<Subtypes>) -> Term {
        Term::Pred(pred)
    }
}

impl From<Succ<Subtypes>> for Term {
    fn from(succ: Succ<Subtypes>) -> Term {
        Term::Succ(succ)
    }
}

impl From<Let<Subtypes>> for Term {
    fn from(lt: Let<Subtypes>) -> Term {
        Term::Let(lt)
    }
}

impl From<RecordProj<Subtypes>> for Term {
    fn from(proj: RecordProj<Subtypes>) -> Term {
        Term::RecordProj(proj)
    }
}

impl From<VariantCase<Subtypes>> for Term {
    fn from(case: VariantCase<Subtypes>) -> Term {
        Term::VariantCase(case)
    }
}

impl From<Fix<Subtypes>> for Term {
    fn from(fix: Fix<Subtypes>) -> Term {
        Term::Fix(fix)
    }
}
