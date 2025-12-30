use super::Existential;
use macros::{
    FreeTypeVars, FromVariants, GenerateConstraintsType, GrammarDescribe, LangDisplay, LatexFmt,
    NoKinds, NoNorm, NoSubtypes, SolveConstraint, Spanned, SubstType,
};
use syntax::types::{
    Bool, Exists, Fun, Nat, Record, Type as TypeTrait, TypeGroup, TypeVariable, Unit,
};

#[derive(
    SolveConstraint,
    GenerateConstraintsType,
    FreeTypeVars,
    Spanned,
    GrammarDescribe,
    FromVariants,
    SubstType,
    LatexFmt,
    LangDisplay,
    NoNorm,
    NoKinds,
    NoSubtypes,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[Lang(Existential)]
pub enum Type {
    Var(TypeVariable<Existential>),
    Unit(Unit<Existential>),
    Nat(Nat<Existential>),
    Bool(Bool<Existential>),
    Fun(Fun<Existential>),
    Exists(Exists<Existential>),
    Record(Record<Existential>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = Existential;
    fn into_unit(self) -> Option<Unit<Existential>> {
        if let Self::Unit(u) = self {
            Some(u)
        } else {
            None
        }
    }

    fn into_nat(self) -> Option<Nat<Existential>> {
        if let Self::Nat(nat) = self {
            Some(nat)
        } else {
            None
        }
    }

    fn into_bool(self) -> Option<Bool<Existential>> {
        if let Self::Bool(b) = self {
            Some(b)
        } else {
            None
        }
    }

    fn into_fun(self) -> Option<Fun<Existential>> {
        if let Self::Fun(fun) = self {
            Some(fun)
        } else {
            None
        }
    }

    fn into_exists(self) -> Option<Exists<Existential>> {
        if let Self::Exists(ex) = self {
            Some(ex)
        } else {
            None
        }
    }

    fn into_record(self) -> Option<Record<Existential>> {
        if let Self::Record(rec) = self {
            Some(rec)
        } else {
            None
        }
    }
}
