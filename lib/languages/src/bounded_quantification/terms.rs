use super::{BoundedQuantification, types::Type};
use macros::{Eval, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, Typecheck};
use syntax::{
    TypeVar, Var,
    subst::SubstType,
    terms::{
        App, Lambda, LambdaSub, Num, Pack, Pred, Record, RecordProj, Succ, TyApp, Unpack, Variable,
    },
};

#[derive(
    SubstTerm, LatexFmt, LangDisplay, GrammarDescribe, Eval, Typecheck, Clone, Debug, PartialEq, Eq,
)]
#[Lang(BoundedQuantification)]
pub enum Term {
    Variable(Variable<BoundedQuantification>),
    Num(Num<BoundedQuantification>),
    Succ(Succ<BoundedQuantification>),
    Pred(Pred<BoundedQuantification>),
    Lambda(Lambda<BoundedQuantification>),
    App(App<BoundedQuantification>),
    LambdaSub(LambdaSub<BoundedQuantification>),
    TyApp(TyApp<BoundedQuantification>),
    Pack(Pack<BoundedQuantification>),
    Unpack(Unpack<BoundedQuantification>),
    Record(Record<BoundedQuantification>),
    RecordProj(RecordProj<BoundedQuantification>),
}

impl syntax::terms::Term for Term {}

impl SubstType for Term {
    type Lang = BoundedQuantification;
    type Target = Term;
    fn subst_type(self, v: &TypeVar, t: &Type) -> Self::Target {
        match self {
            Term::Variable(var) => var.subst_type(v, t).into(),
            Term::Num(num) => num.subst_type(v, t).into(),
            Term::Succ(succ) => succ.subst_type(v, t).into(),
            Term::Pred(pred) => pred.subst_type(v, t).into(),
            Term::Lambda(lam) => lam.subst_type(v, t).into(),
            Term::App(app) => app.subst_type(v, t).into(),
            Term::LambdaSub(lam) => lam.subst_type(v, t).into(),
            Term::TyApp(app) => app.subst_type(v, t).into(),
            Term::Pack(pack) => pack.subst_type(v, t).into(),
            Term::Unpack(unpack) => unpack.subst_type(v, t).into(),
            Term::Record(rec) => rec.subst_type(v, t).into(),
            Term::RecordProj(proj) => proj.subst_type(v, t).into(),
        }
    }
}

impl From<Variable<BoundedQuantification>> for Term {
    fn from(var: Variable<BoundedQuantification>) -> Term {
        Term::Variable(var)
    }
}

impl From<Num<BoundedQuantification>> for Term {
    fn from(num: Num<BoundedQuantification>) -> Term {
        Term::Num(num)
    }
}

impl From<Succ<BoundedQuantification>> for Term {
    fn from(succ: Succ<BoundedQuantification>) -> Term {
        Term::Succ(succ)
    }
}

impl From<Pred<BoundedQuantification>> for Term {
    fn from(pred: Pred<BoundedQuantification>) -> Term {
        Term::Pred(pred)
    }
}

impl From<Lambda<BoundedQuantification>> for Term {
    fn from(lam: Lambda<BoundedQuantification>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<App<BoundedQuantification>> for Term {
    fn from(app: App<BoundedQuantification>) -> Term {
        Term::App(app)
    }
}

impl From<LambdaSub<BoundedQuantification>> for Term {
    fn from(lam: LambdaSub<BoundedQuantification>) -> Term {
        Term::LambdaSub(lam)
    }
}

impl From<TyApp<BoundedQuantification>> for Term {
    fn from(app: TyApp<BoundedQuantification>) -> Term {
        Term::TyApp(app)
    }
}

impl From<Pack<BoundedQuantification>> for Term {
    fn from(pack: Pack<BoundedQuantification>) -> Term {
        Term::Pack(pack)
    }
}

impl From<Unpack<BoundedQuantification>> for Term {
    fn from(unpack: Unpack<BoundedQuantification>) -> Term {
        Term::Unpack(unpack)
    }
}

impl From<Record<BoundedQuantification>> for Term {
    fn from(rec: Record<BoundedQuantification>) -> Term {
        Term::Record(rec)
    }
}

impl From<RecordProj<BoundedQuantification>> for Term {
    fn from(proj: RecordProj<BoundedQuantification>) -> Term {
        Term::RecordProj(proj)
    }
}
