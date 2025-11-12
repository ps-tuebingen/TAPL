use check::Normalize;
use derivations::{Derivation, NormalizingDerivation};
use grammar::DerivationRule;
use macros::{Kindcheck, Subtypecheck, Typecheck};
use std::{collections::HashSet, fmt};
use syntax::{
    TypeVar, Var,
    env::Environment,
    language::{Language, LanguageFeatures},
    subst::{SubstTerm, SubstType},
    terms::{False, Num, Term, True},
    types::{Bool, Nat, Top, Type, TypeGroup},
    values::{Value, ValueGroup},
};

#[derive(Debug, Clone, PartialEq)]
struct DummyLang;

#[derive(Typecheck, Debug, Clone, PartialEq)]
#[Lang(DummyLang)]
enum DummyTerm {
    Num(Num<DummyLang>),
    True(True<DummyLang>),
    False(False<DummyLang>),
}

#[derive(Kindcheck, Subtypecheck, Debug, Clone, PartialEq)]
#[Lang(DummyLang)]
enum DummyType {
    Nat(Nat<DummyLang>),
    Bool(Bool<DummyLang>),
    Top(Top<DummyLang>),
}
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

impl Normalize for DummyType {
    type Lang = DummyLang;
    fn normalize(self, _: Environment<Self::Lang>) -> Derivation<Self::Lang> {
        NormalizingDerivation::empty(self).into()
    }

    fn rules() -> HashSet<DerivationRule> {
        HashSet::new()
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
