use super::{Stlc, types::Type};
use latex::{LatexConfig, LatexFmt};
use macros::{Eval, GrammarDescribe, LangDisplay, Typecheck};
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{
        App, Ascribe, Cons, False, Fix, Fst, Head, If, IsNil, IsZero, Lambda, Left, Let, Nil,
        Nothing, Num, Pair, Pred, Projection, Record, RecordProj, Right, Snd, SomeCase, Something,
        Succ, SumCase, Tail, True, Tuple, Unit, Variable, Variant, VariantCase,
    },
};

#[derive(LangDisplay, GrammarDescribe, Eval, Typecheck, Debug, Clone, PartialEq, Eq)]
#[Lang(Stlc)]
pub enum Term {
    Variable(Variable<Stlc>),
    Lambda(Lambda<Stlc>),
    App(App<Stlc>),
    Unit(Unit<Stlc>),
    True(True<Stlc>),
    False(False<Stlc>),
    If(If<Stlc>),
    Num(Num<Stlc>),
    Pred(Pred<Stlc>),
    Succ(Succ<Stlc>),
    IsZero(IsZero<Stlc>),
    Ascribe(Ascribe<Stlc>),
    Let(Let<Stlc>),
    Pair(Pair<Stlc>),
    Fst(Fst<Stlc>),
    Snd(Snd<Stlc>),
    Tuple(Tuple<Stlc>),
    Projection(Projection<Stlc>),
    Record(Record<Stlc>),
    RecordProj(RecordProj<Stlc>),
    Left(Left<Stlc>),
    Right(Right<Stlc>),
    SumCase(SumCase<Stlc>),
    Variant(Variant<Stlc>),
    VariantCase(VariantCase<Stlc>),
    Nothing(Nothing<Stlc>),
    Something(Something<Stlc>),
    SomeCase(SomeCase<Stlc>),
    Fix(Fix<Stlc>),
    Nil(Nil<Stlc>),
    Cons(Cons<Stlc>),
    IsNil(IsNil<Stlc>),
    Head(Head<Stlc>),
    Tail(Tail<Stlc>),
}

impl syntax::terms::Term for Term {}

impl SubstType for Term {
    type Lang = Stlc;
    type Target = Term;
    fn subst_type(self, _: &TypeVar, _: &Type) -> Self::Target {
        self
    }
}

impl SubstTerm for Term {
    type Lang = Stlc;
    type Target = Term;
    fn subst(self, v: &Var, t: &Term) -> Self::Target {
        match self {
            Term::Variable(var) => var.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t).into(),
            Term::App(app) => app.subst(v, t).into(),
            Term::True(tru) => tru.subst(v, t).into(),
            Term::False(fls) => fls.subst(v, t).into(),
            Term::Unit(u) => u.subst(v, t).into(),
            Term::If(ift) => ift.subst(v, t).into(),
            Term::Num(num) => num.subst(v, t).into(),
            Term::Pred(p) => p.subst(v, t).into(),
            Term::Succ(s) => s.subst(v, t).into(),
            Term::IsZero(isz) => isz.subst(v, t).into(),
            Term::Ascribe(asc) => asc.subst(v, t).into(),
            Term::Let(lt) => lt.subst(v, t).into(),
            Term::Pair(pr) => pr.subst(v, t).into(),
            Term::Tuple(tup) => tup.subst(v, t).into(),
            Term::Projection(proj) => proj.subst(v, t).into(),
            Term::Fst(proj) => proj.subst(v, t).into(),
            Term::Snd(proj) => proj.subst(v, t).into(),
            Term::Record(rec) => rec.subst(v, t).into(),
            Term::RecordProj(proj) => proj.subst(v, t).into(),
            Term::Left(lf) => lf.subst(v, t).into(),
            Term::Right(rt) => rt.subst(v, t).into(),
            Term::SumCase(case) => case.subst(v, t).into(),
            Term::Variant(var) => var.subst(v, t).into(),
            Term::VariantCase(case) => case.subst(v, t).into(),
            Term::Nothing(not) => not.subst(v, t).into(),
            Term::Something(some) => some.subst(v, t).into(),
            Term::SomeCase(case) => case.subst(v, t).into(),
            Term::Fix(fix) => fix.subst(v, t).into(),
            Term::Nil(nil) => nil.subst(v, t).into(),
            Term::Cons(cons) => cons.subst(v, t).into(),
            Term::IsNil(isnil) => isnil.subst(v, t).into(),
            Term::Head(hd) => hd.subst(v, t).into(),
            Term::Tail(tl) => tl.subst(v, t).into(),
        }
    }
}

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Variable(v) => v.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::Unit(unit) => unit.to_latex(conf),
            Term::True(tru) => tru.to_latex(conf),
            Term::False(fls) => fls.to_latex(conf),
            Term::If(ift) => ift.to_latex(conf),
            Term::Num(num) => num.to_latex(conf),
            Term::Pred(p) => p.to_latex(conf),
            Term::Succ(s) => s.to_latex(conf),
            Term::IsZero(isz) => isz.to_latex(conf),
            Term::Ascribe(asc) => asc.to_latex(conf),
            Term::Let(lt) => lt.to_latex(conf),
            Term::Pair(pr) => pr.to_latex(conf),
            Term::Tuple(tup) => tup.to_latex(conf),
            Term::Projection(proj) => proj.to_latex(conf),
            Term::Fst(proj) => proj.to_latex(conf),
            Term::Snd(proj) => proj.to_latex(conf),
            Term::Record(rec) => rec.to_latex(conf),
            Term::RecordProj(proj) => proj.to_latex(conf),
            Term::Left(lf) => lf.to_latex(conf),
            Term::Right(rt) => rt.to_latex(conf),
            Term::SumCase(case) => case.to_latex(conf),
            Term::Variant(var) => var.to_latex(conf),
            Term::VariantCase(case) => case.to_latex(conf),
            Term::Nothing(not) => not.to_latex(conf),
            Term::Something(some) => some.to_latex(conf),
            Term::SomeCase(case) => case.to_latex(conf),
            Term::Fix(fix) => fix.to_latex(conf),
            Term::Nil(nil) => nil.to_latex(conf),
            Term::Cons(cons) => cons.to_latex(conf),
            Term::IsNil(isnil) => isnil.to_latex(conf),
            Term::Head(hd) => hd.to_latex(conf),
            Term::Tail(tl) => tl.to_latex(conf),
        }
    }
}

impl From<Lambda<Stlc>> for Term {
    fn from(lam: Lambda<Stlc>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<Unit<Stlc>> for Term {
    fn from(u: Unit<Stlc>) -> Term {
        Term::Unit(u)
    }
}

impl From<True<Stlc>> for Term {
    fn from(tru: True<Stlc>) -> Term {
        Term::True(tru)
    }
}

impl From<False<Stlc>> for Term {
    fn from(fls: False<Stlc>) -> Term {
        Term::False(fls)
    }
}

impl From<Num<Stlc>> for Term {
    fn from(num: Num<Stlc>) -> Term {
        Term::Num(num)
    }
}

impl From<Pair<Stlc>> for Term {
    fn from(pair: Pair<Stlc>) -> Term {
        Term::Pair(pair)
    }
}

impl From<Tuple<Stlc>> for Term {
    fn from(tup: Tuple<Stlc>) -> Term {
        Term::Tuple(tup)
    }
}

impl From<Record<Stlc>> for Term {
    fn from(rec: Record<Stlc>) -> Term {
        Term::Record(rec)
    }
}

impl From<Left<Stlc>> for Term {
    fn from(lft: Left<Stlc>) -> Term {
        Term::Left(lft)
    }
}

impl From<Right<Stlc>> for Term {
    fn from(right: Right<Stlc>) -> Term {
        Term::Right(right)
    }
}

impl From<Variant<Stlc>> for Term {
    fn from(var: Variant<Stlc>) -> Term {
        Term::Variant(var)
    }
}

impl From<Nothing<Stlc>> for Term {
    fn from(not: Nothing<Stlc>) -> Term {
        Term::Nothing(not)
    }
}

impl From<Something<Stlc>> for Term {
    fn from(some: Something<Stlc>) -> Term {
        Term::Something(some)
    }
}

impl From<Nil<Stlc>> for Term {
    fn from(nil: Nil<Stlc>) -> Term {
        Term::Nil(nil)
    }
}

impl From<Cons<Stlc>> for Term {
    fn from(cons: Cons<Stlc>) -> Term {
        Term::Cons(cons)
    }
}

impl From<Variable<Stlc>> for Term {
    fn from(var: Variable<Stlc>) -> Term {
        Term::Variable(var)
    }
}

impl From<App<Stlc>> for Term {
    fn from(app: App<Stlc>) -> Term {
        Term::App(app)
    }
}

impl From<If<Stlc>> for Term {
    fn from(ift: If<Stlc>) -> Term {
        Term::If(ift)
    }
}

impl From<Pred<Stlc>> for Term {
    fn from(pred: Pred<Stlc>) -> Term {
        Term::Pred(pred)
    }
}

impl From<Succ<Stlc>> for Term {
    fn from(succ: Succ<Stlc>) -> Term {
        Term::Succ(succ)
    }
}

impl From<IsZero<Stlc>> for Term {
    fn from(isz: IsZero<Stlc>) -> Term {
        Term::IsZero(isz)
    }
}

impl From<Ascribe<Stlc>> for Term {
    fn from(asc: Ascribe<Stlc>) -> Term {
        Term::Ascribe(asc)
    }
}

impl From<Let<Stlc>> for Term {
    fn from(lt: Let<Stlc>) -> Term {
        Term::Let(lt)
    }
}

impl From<Projection<Stlc>> for Term {
    fn from(proj: Projection<Stlc>) -> Term {
        Term::Projection(proj)
    }
}

impl From<Fst<Stlc>> for Term {
    fn from(fst: Fst<Stlc>) -> Term {
        Term::Fst(fst)
    }
}

impl From<Snd<Stlc>> for Term {
    fn from(snd: Snd<Stlc>) -> Term {
        Term::Snd(snd)
    }
}

impl From<RecordProj<Stlc>> for Term {
    fn from(proj: RecordProj<Stlc>) -> Term {
        Term::RecordProj(proj)
    }
}

impl From<SumCase<Stlc>> for Term {
    fn from(case: SumCase<Stlc>) -> Term {
        Term::SumCase(case)
    }
}

impl From<VariantCase<Stlc>> for Term {
    fn from(case: VariantCase<Stlc>) -> Term {
        Term::VariantCase(case)
    }
}

impl From<SomeCase<Stlc>> for Term {
    fn from(case: SomeCase<Stlc>) -> Term {
        Term::SomeCase(case)
    }
}

impl From<Fix<Stlc>> for Term {
    fn from(fix: Fix<Stlc>) -> Term {
        Term::Fix(fix)
    }
}

impl From<IsNil<Stlc>> for Term {
    fn from(isn: IsNil<Stlc>) -> Term {
        Term::IsNil(isn)
    }
}

impl From<Head<Stlc>> for Term {
    fn from(hd: Head<Stlc>) -> Term {
        Term::Head(hd)
    }
}

impl From<Tail<Stlc>> for Term {
    fn from(tl: Tail<Stlc>) -> Term {
        Term::Tail(tl)
    }
}
