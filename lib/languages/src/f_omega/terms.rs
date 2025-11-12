use super::FOmega;
use macros::{Eval, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType, Typecheck};
use syntax::terms::{
    App, False, Fix, If, IsZero, Lambda, Num, Pack, Pred, Record, RecordProj, Succ, True, TyApp,
    TyLambda, Unit, Unpack, Variable,
};

#[derive(
    SubstType,
    SubstTerm,
    LatexFmt,
    LangDisplay,
    GrammarDescribe,
    Eval,
    Typecheck,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[Lang(FOmega)]
pub enum Term {
    Variable(Variable<FOmega>),
    Lambda(Lambda<FOmega>),
    App(App<FOmega>),
    TyLambda(TyLambda<FOmega>),
    TyApp(TyApp<FOmega>),
    Pack(Pack<FOmega>),
    Unpack(Unpack<FOmega>),
    Record(Record<FOmega>),
    RecordProj(RecordProj<FOmega>),
    True(True<FOmega>),
    False(False<FOmega>),
    If(If<FOmega>),
    Unit(Unit<FOmega>),
    Fix(Fix<FOmega>),
    Num(Num<FOmega>),
    Succ(Succ<FOmega>),
    Pred(Pred<FOmega>),
    IsZero(IsZero<FOmega>),
}

impl syntax::terms::Term for Term {}

impl From<Pack<FOmega>> for Term {
    fn from(pack: Pack<FOmega>) -> Term {
        Term::Pack(pack)
    }
}
impl From<Unpack<FOmega>> for Term {
    fn from(unpack: Unpack<FOmega>) -> Term {
        Term::Unpack(unpack)
    }
}
impl From<TyApp<FOmega>> for Term {
    fn from(tyapp: TyApp<FOmega>) -> Term {
        Term::TyApp(tyapp)
    }
}

impl From<TyLambda<FOmega>> for Term {
    fn from(tylam: TyLambda<FOmega>) -> Term {
        Term::TyLambda(tylam)
    }
}
impl From<Lambda<FOmega>> for Term {
    fn from(lam: Lambda<FOmega>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<Unit<FOmega>> for Term {
    fn from(u: Unit<FOmega>) -> Term {
        Term::Unit(u)
    }
}

impl From<True<FOmega>> for Term {
    fn from(tru: True<FOmega>) -> Term {
        Term::True(tru)
    }
}

impl From<False<FOmega>> for Term {
    fn from(fls: False<FOmega>) -> Term {
        Term::False(fls)
    }
}

impl From<Num<FOmega>> for Term {
    fn from(num: Num<FOmega>) -> Term {
        Term::Num(num)
    }
}

impl From<Record<FOmega>> for Term {
    fn from(rec: Record<FOmega>) -> Term {
        Term::Record(rec)
    }
}

impl From<Variable<FOmega>> for Term {
    fn from(var: Variable<FOmega>) -> Term {
        Term::Variable(var)
    }
}

impl From<App<FOmega>> for Term {
    fn from(app: App<FOmega>) -> Term {
        Term::App(app)
    }
}

impl From<If<FOmega>> for Term {
    fn from(ift: If<FOmega>) -> Term {
        Term::If(ift)
    }
}

impl From<Pred<FOmega>> for Term {
    fn from(pred: Pred<FOmega>) -> Term {
        Term::Pred(pred)
    }
}

impl From<Succ<FOmega>> for Term {
    fn from(succ: Succ<FOmega>) -> Term {
        Term::Succ(succ)
    }
}

impl From<IsZero<FOmega>> for Term {
    fn from(isz: IsZero<FOmega>) -> Term {
        Term::IsZero(isz)
    }
}

impl From<RecordProj<FOmega>> for Term {
    fn from(proj: RecordProj<FOmega>) -> Term {
        Term::RecordProj(proj)
    }
}

impl From<Fix<FOmega>> for Term {
    fn from(fix: Fix<FOmega>) -> Term {
        Term::Fix(fix)
    }
}
