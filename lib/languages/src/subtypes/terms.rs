use super::Subtypes;
use macros::{
    Eval, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType, Typecheck,
};
use syntax::terms::{
    App, Assign, Cast, Cons, Deref, False, Fix, If, Lambda, Let, ListCase, Loc, Nil, Num, Pred,
    Record, RecordProj, Ref, Succ, True, Unit, Variable, Variant, VariantCase,
};

#[derive(
    FromVariants,
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
