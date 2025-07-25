use super::types::Type;
use grammar::{Grammar, GrammarDescribe, RuleDescribe};
use latex::{LatexConfig, LatexFmt};
use std::fmt;
use syntax::{
    TypeVar, Var,
    subst::{SubstTerm, SubstType},
    terms::{
        App, Lambda, LambdaSub, Let, Num, Pack, Pred, Record, RecordProj, Succ, TyApp, Unpack,
        Variable,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Var(Variable<Term>),
    Lambda(Lambda<Term, Type>),
    App(App<Term>),
    LambdaSub(LambdaSub<Term, Type>),
    TyApp(TyApp<Term, Type>),
    Pack(Pack<Term, Type>),
    Unpack(Unpack<Term, Type>),
    Record(Record<Term>),
    RecordProj(RecordProj<Term>),
    Num(Num<Term>),
    Succ(Succ<Term>),
    Pred(Pred<Term>),
    Let(Let<Term>),
}

impl syntax::terms::Term for Term {}

impl GrammarDescribe for Term {
    fn grammar() -> Grammar {
        Grammar::term(vec![
            Variable::<Term>::rule(),
            Lambda::<Term, Type>::rule(),
            App::<Term>::rule(),
            LambdaSub::<Term, Type>::rule(),
            TyApp::<Term, Type>::rule(),
            Pack::<Term, Type>::rule(),
            Unpack::<Term, Type>::rule(),
            Record::<Term>::rule(),
            RecordProj::<Term>::rule(),
            Num::<Term>::rule(),
            Succ::<Term>::rule(),
            Pred::<Term>::rule(),
            Let::<Term>::rule(),
        ])
    }
}

impl SubstTerm<Term> for Term {
    type Target = Self;

    fn subst(self, v: &Var, t: &Term) -> Term {
        match self {
            Term::Var(var) => var.subst(v, t),
            Term::Lambda(lam) => lam.subst(v, t),
            Term::App(app) => app.subst(v, t),
            Term::LambdaSub(lam) => lam.subst(v, t),
            Term::TyApp(app) => app.subst(v, t),
            Term::Pack(pack) => pack.subst(v, t),
            Term::Unpack(unpack) => unpack.subst(v, t),
            Term::Record(rec) => rec.subst(v, t),
            Term::RecordProj(proj) => proj.subst(v, t),
            Term::Num(num) => num.subst(v, t),
            Term::Succ(succ) => succ.subst(v, t),
            Term::Pred(pred) => pred.subst(v, t),
            Term::Let(lt) => lt.subst(v, t),
        }
    }
}

impl SubstType<Type> for Term {
    type Target = Self;

    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self {
        match self {
            Term::Var(var) => var.subst_type(v, ty),
            Term::Lambda(lam) => lam.subst_type(v, ty),
            Term::App(app) => app.subst_type(v, ty),
            Term::LambdaSub(lam) => lam.subst_type(v, ty),
            Term::TyApp(app) => app.subst_type(v, ty),
            Term::Pack(pack) => pack.subst_type(v, ty),
            Term::Unpack(unpack) => unpack.subst_type(v, ty),
            Term::Record(rec) => rec.subst_type(v, ty),
            Term::RecordProj(proj) => proj.subst_type(v, ty),
            Term::Num(num) => num.subst_type(v, ty),
            Term::Succ(succ) => succ.subst_type(v, ty),
            Term::Pred(pred) => pred.subst_type(v, ty),
            Term::Let(lt) => lt.subst_type(v, ty),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Var(var) => var.fmt(f),
            Term::Lambda(lam) => lam.fmt(f),
            Term::App(app) => app.fmt(f),
            Term::LambdaSub(lam) => lam.fmt(f),
            Term::TyApp(app) => app.fmt(f),
            Term::Pack(pack) => pack.fmt(f),
            Term::Unpack(unpack) => unpack.fmt(f),
            Term::Record(rec) => rec.fmt(f),
            Term::RecordProj(proj) => proj.fmt(f),
            Term::Num(num) => num.fmt(f),
            Term::Succ(succ) => succ.fmt(f),
            Term::Pred(pred) => pred.fmt(f),
            Term::Let(lt) => lt.fmt(f),
        }
    }
}

impl LatexFmt for Term {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Term::Var(var) => var.to_latex(conf),
            Term::Lambda(lam) => lam.to_latex(conf),
            Term::App(app) => app.to_latex(conf),
            Term::LambdaSub(lam) => lam.to_latex(conf),
            Term::TyApp(app) => app.to_latex(conf),
            Term::Pack(pack) => pack.to_latex(conf),
            Term::Unpack(unpack) => unpack.to_latex(conf),
            Term::Record(rec) => rec.to_latex(conf),
            Term::RecordProj(proj) => proj.to_latex(conf),
            Term::Num(num) => num.to_latex(conf),
            Term::Succ(succ) => succ.to_latex(conf),
            Term::Pred(pred) => pred.to_latex(conf),
            Term::Let(lt) => lt.to_latex(conf),
        }
    }
}

impl From<Let<Term>> for Term {
    fn from(lt: Let<Term>) -> Term {
        Term::Let(lt)
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
impl From<TyApp<Term, Type>> for Term {
    fn from(tyapp: TyApp<Term, Type>) -> Term {
        Term::TyApp(tyapp)
    }
}

impl From<Lambda<Term, Type>> for Term {
    fn from(lam: Lambda<Term, Type>) -> Term {
        Term::Lambda(lam)
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
impl From<LambdaSub<Term, Type>> for Term {
    fn from(lam: LambdaSub<Term, Type>) -> Term {
        Term::LambdaSub(lam)
    }
}
impl From<RecordProj<Term>> for Term {
    fn from(proj: RecordProj<Term>) -> Term {
        Term::RecordProj(proj)
    }
}
