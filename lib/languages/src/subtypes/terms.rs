use super::Subtypes;
use macros::{Eval, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType, Typecheck};
use syntax::terms::{
    App, Assign, Cast, Cons, Deref, False, Fix, If, Lambda, Let, ListCase, Loc, Nil, Num, Pred,
    Record, RecordProj, Ref, Succ, True, Unit, Variable, Variant, VariantCase,
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
    PartialEq,
    Eq,
    Clone,
)]
#[Lang(Subtypes)]
pub enum Term {
    Variable(Variable<Subtypes>),
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
        Term::Variable(var)
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
