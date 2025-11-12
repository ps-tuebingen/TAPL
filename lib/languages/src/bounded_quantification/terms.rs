use super::BoundedQuantification;
use macros::{
    Eval, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType, Typecheck,
};
use syntax::terms::{
    App, Lambda, LambdaSub, Num, Pack, Pred, Record, RecordProj, Succ, TyApp, Unpack, Variable,
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
    Clone,
    Debug,
    PartialEq,
    Eq,
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
