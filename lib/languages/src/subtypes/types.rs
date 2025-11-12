use super::Subtypes;
use errors::TypeMismatch;
use grammar::{Grammar, GrammarDescribe, GrammarRuleDescribe};
use latex::{LatexConfig, LatexFmt};
use macros::{NoKinds, NoNorm, Subtypecheck};
use std::fmt;
use syntax::{
    TypeVar,
    subst::SubstType,
    types::{
        Bool, Bot, Fun, List, Nat, Record, Reference, Sink, Source, Top, Type as TypeTrait,
        TypeGroup, Unit, Variant,
    },
};

#[derive(NoNorm, NoKinds, Subtypecheck, Debug, Clone, PartialEq, Eq)]
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
        if let Type::Unit(u) = self {
            Ok(u)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Unit".to_owned()))
        }
    }
    fn into_top(self) -> Result<Top<Subtypes>, TypeMismatch> {
        if let Type::Top(top) = self {
            Ok(top)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Top".to_owned()))
        }
    }

    fn into_bot(self) -> Result<Bot<Subtypes>, TypeMismatch> {
        if let Type::Bot(bot) = self {
            Ok(bot)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Bot".to_owned()))
        }
    }

    fn into_fun(self) -> Result<Fun<Subtypes>, TypeMismatch> {
        if let Type::Fun(fun) = self {
            Ok(fun)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Function".to_owned()))
        }
    }

    fn into_record(self) -> Result<Record<Subtypes>, TypeMismatch> {
        if let Type::Record(rec) = self {
            Ok(rec)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Record".to_owned()))
        }
    }

    fn into_variant(self) -> Result<Variant<Subtypes>, TypeMismatch> {
        if let Type::Variant(var) = self {
            Ok(var)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Variant".to_owned()))
        }
    }

    fn into_list(self) -> Result<List<Subtypes>, TypeMismatch> {
        if let Type::List(list) = self {
            Ok(list)
        } else {
            Err(TypeMismatch::new(self.to_string(), "List".to_owned()))
        }
    }

    fn into_ref(self) -> Result<Reference<Subtypes>, TypeMismatch> {
        if let Type::Ref(reft) = self {
            Ok(reft)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Reference".to_owned()))
        }
    }

    fn into_source(self) -> Result<Source<Subtypes>, TypeMismatch> {
        if let Type::Source(src) = self {
            Ok(src)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Source".to_owned()))
        }
    }

    fn into_sink(self) -> Result<Sink<Subtypes>, TypeMismatch> {
        if let Type::Sink(sink) = self {
            Ok(sink)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Sink".to_owned()))
        }
    }

    fn into_nat(self) -> Result<Nat<Subtypes>, TypeMismatch> {
        if let Type::Nat(nat) = self {
            Ok(nat)
        } else {
            Err(TypeMismatch::new(self.to_string(), "Nat".to_owned()))
        }
    }

    fn into_bool(self) -> Result<Bool<Subtypes>, TypeMismatch> {
        if let Type::Bool(b) = self {
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

impl SubstType for Type {
    type Lang = Subtypes;
    type Target = Self;
    fn subst_type(self, _: &TypeVar, _: &Type) -> Self::Target {
        self
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Top(top) => top.fmt(f),
            Type::Bot(bot) => bot.fmt(f),
            Type::Fun(fun) => fun.fmt(f),
            Type::Record(rec) => rec.fmt(f),
            Type::Variant(variant) => variant.fmt(f),
            Type::List(list) => list.fmt(f),
            Type::Ref(refty) => refty.fmt(f),
            Type::Source(src) => src.fmt(f),
            Type::Sink(snk) => snk.fmt(f),
            Type::Nat(nat) => nat.fmt(f),
            Type::Unit(unit) => unit.fmt(f),
            Type::Bool(b) => b.fmt(f),
        }
    }
}

impl LatexFmt for Type {
    fn to_latex(&self, conf: &mut LatexConfig) -> String {
        match self {
            Type::Top(top) => top.to_latex(conf),
            Type::Bot(bot) => bot.to_latex(conf),
            Type::Fun(fun) => fun.to_latex(conf),
            Type::Record(rec) => rec.to_latex(conf),
            Type::Variant(variant) => variant.to_latex(conf),
            Type::List(list) => list.to_latex(conf),
            Type::Ref(refty) => refty.to_latex(conf),
            Type::Source(src) => src.to_latex(conf),
            Type::Sink(snk) => snk.to_latex(conf),
            Type::Nat(nat) => nat.to_latex(conf),
            Type::Unit(unit) => unit.to_latex(conf),
            Type::Bool(b) => b.to_latex(conf),
        }
    }
}
impl From<Source<Subtypes>> for Type {
    fn from(src: Source<Subtypes>) -> Type {
        Type::Source(src)
    }
}
impl From<Sink<Subtypes>> for Type {
    fn from(sink: Sink<Subtypes>) -> Type {
        Type::Sink(sink)
    }
}

impl From<Reference<Subtypes>> for Type {
    fn from(reft: Reference<Subtypes>) -> Type {
        Type::Ref(reft)
    }
}
impl From<Bot<Subtypes>> for Type {
    fn from(b: Bot<Subtypes>) -> Type {
        Type::Bot(b)
    }
}
impl From<Top<Subtypes>> for Type {
    fn from(t: Top<Subtypes>) -> Type {
        Type::Top(t)
    }
}

impl From<Unit<Subtypes>> for Type {
    fn from(u: Unit<Subtypes>) -> Type {
        Type::Unit(u)
    }
}

impl From<Fun<Subtypes>> for Type {
    fn from(fun: Fun<Subtypes>) -> Type {
        Type::Fun(fun)
    }
}

impl From<Bool<Subtypes>> for Type {
    fn from(b: Bool<Subtypes>) -> Type {
        Type::Bool(b)
    }
}

impl From<Nat<Subtypes>> for Type {
    fn from(n: Nat<Subtypes>) -> Type {
        Type::Nat(n)
    }
}

impl From<Record<Subtypes>> for Type {
    fn from(rec: Record<Subtypes>) -> Type {
        Type::Record(rec)
    }
}

impl From<Variant<Subtypes>> for Type {
    fn from(var: Variant<Subtypes>) -> Type {
        Type::Variant(var)
    }
}

impl From<List<Subtypes>> for Type {
    fn from(ls: List<Subtypes>) -> Type {
        Type::List(ls)
    }
}
