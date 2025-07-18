use super::types::Type;
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
    Var(Variable<Term>),
    Lambda(Lambda<Term, Type>),
    App(App<Term>),
    Unit(Unit<Term>),
    Cast(Cast<Term, Type>),
    Record(Record<Term>),
    RecordProj(RecordProj<Term>),
    Variant(Variant<Term, Type>),
    VariantCase(VariantCase<Term>),
    Nil(Nil<Term, Type>),
    Cons(Cons<Term, Type>),
    ListCase(ListCase<Term>),
    Ref(Ref<Term>),
    Deref(Deref<Term>),
    Assign(Assign<Term>),
    Loc(Loc<Term>),
    Num(Num<Term>),
    Succ(Succ<Term>),
    Pred(Pred<Term>),
    True(True<Term>),
    False(False<Term>),
    If(If<Term>),
    Let(Let<Term>),
    Fix(Fix<Term>),
}

impl syntax::terms::Term for Term {}

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            Variable::<Term>::rule(),
            Lambda::<Term, Type>::rule(),
            App::<Term>::rule(),
            Unit::<Term>::rule(),
            Record::<Term>::rule(),
            RecordProj::<Term>::rule(),
            Variant::<Term, Type>::rule(),
            VariantCase::<Term>::rule(),
            Cast::<Term, Type>::rule(),
            Nil::<Term, Type>::rule(),
            Cons::<Term, Type>::rule(),
            ListCase::<Term>::rule(),
            Ref::<Term>::rule(),
            Deref::<Term>::rule(),
            Assign::<Term>::rule(),
            Loc::<Term>::rule(),
            Num::<Term>::rule(),
            Succ::<Term>::rule(),
            Pred::<Term>::rule(),
            True::<Term>::rule(),
            False::<Term>::rule(),
            If::<Term>::rule(),
            Let::<Term>::rule(),
            Fix::<Term>::rule(),
        ])
    }
}

impl SubstType<Type> for Term {
    type Target = Term;
    fn subst_type(self, _: &TypeVar, _: &Type) -> Self::Target {
        self
    }
}

impl SubstTerm<Term> for Term {
    type Target = Self;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::Unit(unit) => unit.subst(v, t),
            Term::Record(rec) => rec.subst(v, t),
            Term::RecordProj(proj) => proj.subst(v, t),
            Term::Variant(var) => var.subst(v, t),
            Term::VariantCase(case) => case.subst(v, t),
            Term::Cast(cast) => cast.subst(v, t),
            Term::Nil(nil) => nil.subst(v, t),
            Term::Cons(cons) => cons.subst(v, t),
            Term::ListCase(case) => case.subst(v, t),
            Term::Ref(rf) => rf.subst(v, t),
            Term::Deref(deref) => deref.subst(v, t),
            Term::Assign(assign) => assign.subst(v, t),
            Term::Loc(loc) => loc.subst(v, t),
            Term::Num(num) => num.subst(v, t),
            Term::Succ(succ) => succ.subst(v, t),
            Term::Pred(pred) => pred.subst(v, t),
            Term::True(tru) => tru.subst(v, t),
            Term::False(fls) => fls.subst(v, t),
            Term::If(ift) => ift.subst(v, t),
            Term::Let(lt) => lt.subst(v, t),
            Term::Fix(fix) => fix.subst(v, t),
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

impl From<Loc<Term>> for Term {
    fn from(loc: Loc<Term>) -> Term {
        Term::Loc(loc)
    }
}
impl From<Assign<Term>> for Term {
    fn from(assign: Assign<Term>) -> Term {
        Term::Assign(assign)
    }
}
impl From<Deref<Term>> for Term {
    fn from(deref: Deref<Term>) -> Term {
        Term::Deref(deref)
    }
}
impl From<Ref<Term>> for Term {
    fn from(reft: Ref<Term>) -> Term {
        Term::Ref(reft)
    }
}
impl From<ListCase<Term>> for Term {
    fn from(case: ListCase<Term>) -> Term {
        Term::ListCase(case)
    }
}

impl From<Cast<Term, Type>> for Term {
    fn from(cast: Cast<Term, Type>) -> Term {
        Term::Cast(cast)
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

impl From<Nil<Term, Type>> for Term {
    fn from(nil: Nil<Term, Type>) -> Term {
        Term::Nil(nil)
    }
}

impl From<Cons<Term, Type>> for Term {
    fn from(cons: Cons<Term, Type>) -> Term {
        Term::Cons(cons)
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

impl From<Let<Term>> for Term {
    fn from(lt: Let<Term>) -> Term {
        Term::Let(lt)
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
