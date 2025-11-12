use super::FOmega;
use macros::{
    Eval, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType, Typecheck,
};
use syntax::terms::{
    App, False, Fix, If, IsZero, Lambda, Num, Pack, Pred, Record, RecordProj, Succ, True, TyApp,
    TyLambda, Unit, Unpack, Variable,
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
