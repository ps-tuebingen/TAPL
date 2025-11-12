use macros::{Eval, GrammarDescribe, Kindcheck, Normalize, Subtypecheck, Typecheck};
use std::fmt;
use syntax::{
    TypeVar, Var,
    language::{Language, LanguageFeatures},
    subst::{SubstTerm, SubstType},
    terms::{False, Num, Term, True},
    types::{Bool, Nat, Top, Type, TypeGroup},
    values::{False as FalseVal, Num as NumVal, True as TrueVal, Value, ValueGroup},
};

#[derive(Debug, Clone, PartialEq)]
struct DummyLang;

#[derive(GrammarDescribe, Eval, Typecheck, Debug, Clone, PartialEq)]
#[Lang(DummyLang)]
enum DummyTerm {
    Num(Num<DummyLang>),
    True(True<DummyLang>),
    False(False<DummyLang>),
}

#[derive(Normalize, Kindcheck, Subtypecheck, Debug, Clone, PartialEq)]
#[Lang(DummyLang)]
enum DummyType {
    Nat(Nat<DummyLang>),
    Bool(Bool<DummyLang>),
    Top(Top<DummyLang>),
}
#[derive(Clone, Debug)]
enum DummyValue {
    Num(NumVal<DummyLang>),
    True(TrueVal<DummyLang>),
    False(FalseVal<DummyLang>),
}

impl Term for DummyTerm {}
impl Type for DummyType {}
impl TypeGroup for DummyType {
    type Lang = DummyLang;
}
impl Value for DummyValue {
    type Lang = DummyLang;
    type Term = DummyTerm;
}
impl ValueGroup for DummyValue {}

impl SubstTerm for DummyTerm {
    type Lang = DummyLang;
    type Target = Self;
    fn subst(self, _: &Var, _: &<Self::Lang as Language>::Term) -> Self::Target {
        self
    }
}

impl SubstType for DummyType {
    type Lang = DummyLang;
    type Target = Self;
    fn subst_type(self, _: &TypeVar, _: &<Self::Lang as Language>::Type) -> Self::Target {
        self
    }
}
impl SubstType for DummyTerm {
    type Lang = DummyLang;
    type Target = Self;
    fn subst_type(self, _: &TypeVar, _: &<Self::Lang as Language>::Type) -> Self::Target {
        self
    }
}

impl Language for DummyLang {
    type Term = DummyTerm;
    type Type = DummyType;
    type Value = DummyValue;

    fn describe(&self) -> &str {
        "dummy lang"
    }

    fn features() -> LanguageFeatures {
        LanguageFeatures::default()
    }
}

impl From<DummyValue> for DummyTerm {
    fn from(val: DummyValue) -> DummyTerm {
        match val {
            DummyValue::True(t) => DummyTerm::True(t.into()),
            DummyValue::False(f) => DummyTerm::False(f.into()),
            DummyValue::Num(n) => DummyTerm::Num(n.into()),
        }
    }
}

impl From<Num<DummyLang>> for DummyTerm {
    fn from(num: Num<DummyLang>) -> DummyTerm {
        DummyTerm::Num(num)
    }
}

impl From<True<DummyLang>> for DummyTerm {
    fn from(t: True<DummyLang>) -> DummyTerm {
        DummyTerm::True(t)
    }
}

impl From<False<DummyLang>> for DummyTerm {
    fn from(f: False<DummyLang>) -> DummyTerm {
        DummyTerm::False(f)
    }
}

impl From<Nat<DummyLang>> for DummyType {
    fn from(nat: Nat<DummyLang>) -> DummyType {
        DummyType::Nat(nat)
    }
}

impl From<Bool<DummyLang>> for DummyType {
    fn from(b: Bool<DummyLang>) -> DummyType {
        DummyType::Bool(b)
    }
}

impl From<Top<DummyLang>> for DummyType {
    fn from(t: Top<DummyLang>) -> DummyType {
        DummyType::Top(t)
    }
}

impl From<NumVal<DummyLang>> for DummyValue {
    fn from(num: NumVal<DummyLang>) -> DummyValue {
        DummyValue::Num(num)
    }
}

impl From<TrueVal<DummyLang>> for DummyValue {
    fn from(t: TrueVal<DummyLang>) -> DummyValue {
        DummyValue::True(t)
    }
}

impl From<FalseVal<DummyLang>> for DummyValue {
    fn from(f: FalseVal<DummyLang>) -> DummyValue {
        DummyValue::False(f)
    }
}

impl fmt::Display for DummyLang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("dummy")
    }
}

impl fmt::Display for DummyTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("dummy")
    }
}

impl fmt::Display for DummyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("dummy")
    }
}

impl fmt::Display for DummyValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("dummy")
    }
}
