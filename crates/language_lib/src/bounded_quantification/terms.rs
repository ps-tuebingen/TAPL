use super::types::Type;
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{
        App, Lambda, LambdaSub, Num, Pack, Pred, Record, RecordProj, Succ, TyApp, Unpack, Variable,
    },
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term {
    Var(Variable<Term>),
    Num(Num<Term>),
    Succ(Succ<Term>),
    Pred(Pred<Term>),
    Lambda(Lambda<Term, Type>),
    App(App<Term>),
    LambdaSub(LambdaSub<Term, Type>),
    TyApp(TyApp<Term, Type>),
    Pack(Pack<Term, Type>),
    Unpack(Unpack<Term, Type>),
    Record(Record<Term>),
    Projection(RecordProj<Term>),
}

impl syntax::terms::Term for Term {}

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            Variable::<Term>::rule(),
            Num::<Term>::rule(),
            Succ::<Term>::rule(),
            Pred::<Term>::rule(),
            Lambda::<Term, Type>::rule(),
            App::<Term>::rule(),
            LambdaSub::<Term, Type>::rule(),
            TyApp::<Term, Type>::rule(),
            Pack::<Term, Type>::rule(),
            Unpack::<Term, Type>::rule(),
            Record::<Term>::rule(),
            RecordProj::<Term>::rule(),
        ])
    }
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

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Var(v) => v.to_latex(conf),
            Term::Num(num) => num.to_latex(conf),
            Term::Succ(s) => s.to_latex(conf),
            Term::Pred(p) => p.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::LambdaSub(lam) => lam.to_latex(conf),
            Term::TyApp(app) => app.to_latex(conf),
            Term::Pack(pack) => pack.to_latex(conf),
            Term::Unpack(unpack) => unpack.to_latex(conf),
            Term::Record(rec) => rec.to_latex(conf),
            Term::Projection(proj) => proj.to_latex(conf),
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

impl From<Lambda<Term, Type>> for Term {
    fn from(lam: Lambda<Term, Type>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<App<Term>> for Term {
    fn from(app: App<Term>) -> Term {
        Term::App(app)
    }
}

impl From<LambdaSub<Term, Type>> for Term {
    fn from(lam: LambdaSub<Term, Type>) -> Term {
        Term::LambdaSub(lam)
    }
}

impl From<TyApp<Term, Type>> for Term {
    fn from(app: TyApp<Term, Type>) -> Term {
        Term::TyApp(app)
    }
}

impl From<Pack<Term, Type>> for Term {
    fn from(pack: Pack<Term, Type>) -> Term {
        Term::Pack(pack)
    }
}

impl From<Unpack<Term, Type>> for Term {
    fn from(unpack: Unpack<Term, Type>) -> Term {
        Term::Unpack(unpack)
    }
}

impl From<Record<Term>> for Term {
    fn from(rec: Record<Term>) -> Term {
        Term::Record(rec)
    }
}

impl From<RecordProj<Term>> for Term {
    fn from(proj: RecordProj<Term>) -> Term {
        Term::Projection(proj)
    }
}
