use macros::Typecheck;
use std::fmt;
use syntax::{
    TypeVar, Var,
    language::{Language, LanguageFeatures},
    subst::{SubstTerm, SubstType},
    terms::{Num, Term},
    types::{Nat, Type, TypeGroup},
    values::{Value, ValueGroup},
};

#[derive(Debug, Clone, PartialEq)]
enum DummyLang {}

#[derive(Typecheck, Debug, Clone, PartialEq)]
#[Lang(DummyLang)]
enum DummyTerm {
    Num(Num<DummyLang>),
}
#[derive(Debug, Clone, PartialEq)]
struct DummyType;
#[derive(Clone, Debug)]
struct DummyValue;

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
    fn from(_: DummyValue) -> DummyTerm {
        DummyTerm::Num(Num::new(1))
    }
}

impl From<Num<DummyLang>> for DummyTerm {
    fn from(num: Num<DummyLang>) -> DummyTerm {
        DummyTerm::Num(num)
    }
}

impl From<Nat<DummyLang>> for DummyType {
    fn from(_: Nat<DummyLang>) -> DummyType {
        DummyType
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
