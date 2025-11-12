use super::Recursive;
use macros::{
    Eval, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType, Typecheck,
};
use syntax::terms::{
    App, False, Fix, Fold, Fst, If, IsZero, Lambda, Let, Num, Pair, Pred, Record, RecordProj, Snd,
    Succ, True, Unfold, Unit, Variable, Variant, VariantCase,
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
