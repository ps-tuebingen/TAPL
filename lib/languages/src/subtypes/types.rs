use super::Subtypes;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use macros::{FromVariants, LangDisplay, LatexFmt, NoKinds, NoNorm, SubstType, Subtypecheck};
use syntax::types::{
    Bool, Bot, Fun, List, Nat, Record, Reference, Sink, Source, Top, Type as TypeTrait, TypeGroup,
    Unit, Variant,
};

#[derive(
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
    fn into_unit(self) -> Result<Unit<Subtypes>, TypeMismatch> {
        if let Self::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }
    fn into_top(self) -> Result<Top<Subtypes>, TypeMismatch> {
        if let Self::Top(top) = self {
            Ok(top)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Top".to_owned()))
        }
    }

    fn into_bot(self) -> Result<Bot<Subtypes>, TypeMismatch> {
        if let Self::Bot(bot) = self {
            Ok(bot)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bot".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<Subtypes>, TypeMismatch> {
        if let Self::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_record(self) -> Result<Record<Subtypes>, TypeMismatch> {
        if let Self::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_variant(self) -> Result<Variant<Subtypes>, TypeMismatch> {
        if let Self::Variant(var) = self {
            Ok(var)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Variant".to_owned()))
        }
    }

    fn into_list(self) -> Result<List<Subtypes>, TypeMismatch> {
        if let Self::List(list) = self {
            Ok(list)
        } else {
            Err(TypeMismatch::new(self.to_string(), "List".to_owned()))
        }
    }

    fn into_ref(self) -> Result<Reference<Subtypes>, TypeMismatch> {
        if let Self::Ref(reft) = self {
            Ok(reft)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Reference".to_owned()))
        }
    }

    fn into_source(self) -> Result<Source<Subtypes>, TypeMismatch> {
        if let Self::Source(src) = self {
            Ok(src)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Source".to_owned()))
        }
    }

    fn into_sink(self) -> Result<Sink<Subtypes>, TypeMismatch> {
        if let Self::Sink(sink) = self {
            Ok(sink)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Sink".to_owned()))
        }
    }

    fn into_nat(self) -> Result<Nat<Subtypes>, TypeMismatch> {
        if let Self::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<Subtypes>, TypeMismatch> {
        if let Self::Bool(b) = self {
            Ok(b)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bool".to_owned()))
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
