use super::Stlc;
use macros::{
    Eval, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType, Typecheck,
};
use syntax::terms::{
    App, Ascribe, Cons, False, Fix, Fst, Head, If, IsNil, IsZero, Lambda, Left, Let, Nil, Nothing,
    Num, Pair, Pred, Projection, Record, RecordProj, Right, Snd, SomeCase, Something, Succ,
    SumCase, Tail, True, Tuple, Unit, Variable, Variant, VariantCase,
};

#[derive(
    FromVariants,
    SubstType,
    SubstTerm,
    LangDisplay,
    LatexFmt,
    GrammarDescribe,
    Eval,
    Typecheck,
    Debug,
    Clone,
    PartialEq,
    Eq,
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
