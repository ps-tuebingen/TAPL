use super::{Existential, types::Type};
use macros::{Eval, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, Typecheck};
use syntax::{
    TypeVar, Var,
    subst::SubstType,
    terms::{
        App, False, Fix, If, IsZero, Lambda, Num, Pack, Pred, Record, RecordProj, Succ, True, Unit,
        Unpack, Variable,
    },
};

#[derive(
    SubstTerm, LatexFmt, LangDisplay, GrammarDescribe, Eval, Typecheck, Debug, Clone, PartialEq, Eq,
)]
#[Lang(Existential)]
pub enum Term {
    Variable(Variable<Existential>),
    Unit(Unit<Existential>),
    Lambda(Lambda<Existential>),
    App(App<Existential>),
    Pack(Pack<Existential>),
    Unpack(Unpack<Existential>),
    Num(Num<Existential>),
    Succ(Succ<Existential>),
    Pred(Pred<Existential>),
    IsZero(IsZero<Existential>),
    Record(Record<Existential>),
    RecordProj(RecordProj<Existential>),
    True(True<Existential>),
    False(False<Existential>),
    If(If<Existential>),
    Fix(Fix<Existential>),
}

impl syntax::terms::Term for Term {}

impl SubstType for Term {
    type Lang = Existential;
    type Target = Self;
    fn subst_type(self, v: &TypeVar, ty: &Type) -> Self::Target {
        match self {
            Term::Variable(var) => var.subst_type(v, ty).into(),
            Term::Unit(u) => u.subst_type(v, ty).into(),
            Term::Lambda(lam) => lam.subst_type(v, ty).into(),
            Term::App(app) => app.subst_type(v, ty).into(),
            Term::Pack(pack) => pack.subst_type(v, ty).into(),
            Term::Unpack(unpack) => unpack.subst_type(v, ty).into(),
            Term::Num(num) => num.subst_type(v, ty).into(),
            Term::Succ(succ) => succ.subst_type(v, ty).into(),
            Term::Pred(pred) => pred.subst_type(v, ty).into(),
            Term::IsZero(isz) => isz.subst_type(v, ty).into(),
            Term::Record(rec) => rec.subst_type(v, ty).into(),
            Term::RecordProj(proj) => proj.subst_type(v, ty).into(),
            Term::True(tru) => tru.subst_type(v, ty).into(),
            Term::False(fls) => fls.subst_type(v, ty).into(),
            Term::If(ift) => ift.subst_type(v, ty).into(),
            Term::Fix(fix) => fix.subst_type(v, ty).into(),
        }
    }
}

impl From<Pack<Existential>> for Term {
    fn from(pack: Pack<Existential>) -> Term {
        Term::Pack(pack)
    }
}

impl From<Unpack<Existential>> for Term {
    fn from(unp: Unpack<Existential>) -> Term {
        Term::Unpack(unp)
    }
}
impl From<Lambda<Existential>> for Term {
    fn from(lam: Lambda<Existential>) -> Term {
        Term::Lambda(lam)
    }
}

impl From<Unit<Existential>> for Term {
    fn from(u: Unit<Existential>) -> Term {
        Term::Unit(u)
    }
}

impl From<True<Existential>> for Term {
    fn from(tru: True<Existential>) -> Term {
        Term::True(tru)
    }
}

impl From<False<Existential>> for Term {
    fn from(fls: False<Existential>) -> Term {
        Term::False(fls)
    }
}

impl From<Num<Existential>> for Term {
    fn from(num: Num<Existential>) -> Term {
        Term::Num(num)
    }
}

impl From<Record<Existential>> for Term {
    fn from(rec: Record<Existential>) -> Term {
        Term::Record(rec)
    }
}

impl From<Variable<Existential>> for Term {
    fn from(var: Variable<Existential>) -> Term {
        Term::Variable(var)
    }
}

impl From<App<Existential>> for Term {
    fn from(app: App<Existential>) -> Term {
        Term::App(app)
    }
}

impl From<If<Existential>> for Term {
    fn from(ift: If<Existential>) -> Term {
        Term::If(ift)
    }
}

impl From<Pred<Existential>> for Term {
    fn from(pred: Pred<Existential>) -> Term {
        Term::Pred(pred)
    }
}

impl From<Succ<Existential>> for Term {
    fn from(succ: Succ<Existential>) -> Term {
        Term::Succ(succ)
    }
}

impl From<IsZero<Existential>> for Term {
    fn from(isz: IsZero<Existential>) -> Term {
        Term::IsZero(isz)
    }
}

impl From<RecordProj<Existential>> for Term {
    fn from(proj: RecordProj<Existential>) -> Term {
        Term::RecordProj(proj)
    }
}

impl From<Fix<Existential>> for Term {
    fn from(fix: Fix<Existential>) -> Term {
        Term::Fix(fix)
    }
}
