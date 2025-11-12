use super::Recursive;
use macros::{Eval, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType, Typecheck};
use syntax::terms::{
    App, False, Fix, Fold, Fst, If, IsZero, Lambda, Let, Num, Pair, Pred, Record, RecordProj, Snd,
    Succ, True, Unfold, Unit, Variable, Variant, VariantCase,
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
#[Lang(Recursive)]
pub enum Term {
    Variable(Variable<Recursive>),
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
        Term::Variable(var)
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
