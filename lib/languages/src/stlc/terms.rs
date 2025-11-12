use super::{Stlc, types::Type};
use macros::{Eval, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, Typecheck};
use syntax::{
    TypeVar, Var,
    subst::SubstType,
    terms::{
        App, Ascribe, Cons, False, Fix, Fst, Head, If, IsNil, IsZero, Lambda, Left, Let, Nil,
        Nothing, Num, Pair, Pred, Projection, Record, RecordProj, Right, Snd, SomeCase, Something,
        Succ, SumCase, Tail, True, Tuple, Unit, Variable, Variant, VariantCase,
    },
};

#[derive(
    SubstTerm, LangDisplay, LatexFmt, GrammarDescribe, Eval, Typecheck, Debug, Clone, PartialEq, Eq,
)]
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
