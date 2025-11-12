use super::Existential;
use macros::{
    Eval, FromVariants, GrammarDescribe, LangDisplay, LatexFmt, SubstTerm, SubstType, Typecheck,
};
use syntax::terms::{
    App, False, Fix, If, IsZero, Lambda, Num, Pack, Pred, Record, RecordProj, Succ, True, Unit,
    Unpack, Variable,
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
