use super::Subtypes;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{
    FreeTypeVars, FromVariants, GenerateConstraintsType, LangDisplay, LatexFmt, NoKinds, NoNorm,
    SolveConstraint, Spanned, SubstType, Subtypecheck,
};
use syntax::types::{
    Bool, Bot, Fun, List, Nat, Record, Reference, Sink, Source, Top, Type as TypeTrait, TypeGroup,
    Unit, Variant,
};

#[derive(
    SolveConstraint,
    GenerateConstraintsType,
    FreeTypeVars,
    Spanned,
    FromVariants,
    SubstType,
    LatexFmt,
    LangDisplay,
    NoNorm,
    NoKinds,
    Subtypecheck,
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
#[Lang(Subtypes)]
pub enum Type {
    Top(Top<Subtypes>),
    Bot(Bot<Subtypes>),
    Fun(Fun<Subtypes>),
    Record(Record<Subtypes>),
    Variant(Variant<Subtypes>),
    List(List<Subtypes>),
    Ref(Reference<Subtypes>),
    Source(Source<Subtypes>),
    Sink(Sink<Subtypes>),
    Nat(Nat<Subtypes>),
    Unit(Unit<Subtypes>),
    Bool(Bool<Subtypes>),
}

impl TypeTrait for Type {}

impl TypeGroup for Type {
    type Lang = Subtypes;
    fn into_unit(self) -> Option<Unit<Subtypes>> {
        if let Self::Unit(u) = self {
            Some(u)
        } else {
            None
        }
    }
    fn into_top(self) -> Option<Top<Subtypes>> {
        if let Self::Top(top) = self {
            Some(top)
        } else {
            None
        }
    }

    fn into_bot(self) -> Option<Bot<Subtypes>> {
        if let Self::Bot(bot) = self {
            Some(bot)
        } else {
            None
        }
    }

    fn into_fun(self) -> Option<Fun<Subtypes>> {
        if let Self::Fun(fun) = self {
            Some(fun)
        } else {
            None
        }
    }

    fn into_record(self) -> Option<Record<Subtypes>> {
        if let Self::Record(rec) = self {
            Some(rec)
        } else {
            None
        }
    }

    fn into_variant(self) -> Option<Variant<Subtypes>> {
        if let Self::Variant(var) = self {
            Some(var)
        } else {
            None
        }
    }

    fn into_list(self) -> Option<List<Subtypes>> {
        if let Self::List(list) = self {
            Some(list)
        } else {
            None
        }
    }

    fn into_ref(self) -> Option<Reference<Subtypes>> {
        if let Self::Ref(reft) = self {
            Some(reft)
        } else {
            None
        }
    }

    fn into_source(self) -> Option<Source<Subtypes>> {
        if let Self::Source(src) = self {
            Some(src)
        } else {
            None
        }
    }

    fn into_sink(self) -> Option<Sink<Subtypes>> {
        if let Self::Sink(sink) = self {
            Some(sink)
        } else {
            None
        }
    }

    fn into_nat(self) -> Option<Nat<Subtypes>> {
        if let Self::Nat(nat) = self {
            Some(nat)
        } else {
            None
        }
    }

    fn into_bool(self) -> Option<Bool<Subtypes>> {
        if let Self::Bool(b) = self {
            Some(b)
        } else {
            None
        }
    }
}

impl GrammarDescribe for Type {
    fn grammar() -> Grammar {
        Grammar::ty(vec![
            Top::<Subtypes>::rule(),
            Bot::<Subtypes>::rule(),
            Fun::<Subtypes>::rule(),
            Record::<Subtypes>::rule(),
            Variant::<Subtypes>::rule(),
            List::<Subtypes>::rule(),
            Reference::<Subtypes>::rule(),
            Source::<Subtypes>::rule(),
            Sink::<Subtypes>::rule(),
            Nat::<Subtypes>::rule(),
            Unit::<Subtypes>::rule(),
            Bool::<Subtypes>::rule(),
        ])
    }
}
