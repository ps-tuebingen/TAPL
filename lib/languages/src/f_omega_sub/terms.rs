use super::FOmegaSub;
use macros::{
    Eval, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType, Typecheck,
};
use syntax::terms::{
    App, Lambda, LambdaSub, Let, Num, Pack, Pred, Record, RecordProj, Succ, TyApp, Unpack, Variable,
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
#[Lang(FOmegaSub)]
pub enum Term {
    Variable(Variable<FOmegaSub>),
    Lambda(Lambda<FOmegaSub>),
    App(App<FOmegaSub>),
    LambdaSub(LambdaSub<FOmegaSub>),
    TyApp(TyApp<FOmegaSub>),
    Pack(Pack<FOmegaSub>),
    Unpack(Unpack<FOmegaSub>),
    Record(Record<FOmegaSub>),
    RecordProj(RecordProj<FOmegaSub>),
    Num(Num<FOmegaSub>),
    Succ(Succ<FOmegaSub>),
    Pred(Pred<FOmegaSub>),
    Let(Let<FOmegaSub>),
}

impl syntax::terms::Term for Term {}
