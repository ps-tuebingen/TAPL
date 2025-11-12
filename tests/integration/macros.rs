use macros::{
    Eval, FromVariants, GrammarDescribe, Kindcheck, LangDisplay, LatexFmt, Normalize, SubstTerm,
    SubstType, Subtypecheck, Typecheck,
};
use std::fmt;
use syntax::{
    language::{Language, LanguageFeatures},
    terms::{False, Num, Term, True},
    types::{Bool, Nat, Top, Type, TypeGroup},
    values::{False as FalseVal, Num as NumVal, True as TrueVal, Value, ValueGroup},
};

#[derive(Debug, Clone, PartialEq)]
struct DummyLang;

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
)]
#[Lang(DummyLang)]
enum DummyTerm {
    Num(Num<DummyLang>),
    True(True<DummyLang>),
    False(False<DummyLang>),
}

#[derive(
    GrammarDescribe,
    FromVariants,
    SubstType,
    LatexFmt,
    LangDisplay,
    Normalize,
    Kindcheck,
    Subtypecheck,
    Debug,
    Clone,
    PartialEq,
)]
#[Lang(DummyLang)]
enum DummyType {
    Nat(Nat<DummyLang>),
    Bool(Bool<DummyLang>),
    Top(Top<DummyLang>),
}
#[derive(GrammarDescribe, FromVariants, LatexFmt, LangDisplay, Clone, Debug)]
#[Lang(DummyLang)]
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
            DummyValue::Num(n) => DummyTerm::Num(n.into()),
            DummyValue::True(t) => DummyTerm::True(t.into()),
            DummyValue::False(f) => DummyTerm::False(f.into()),
        }
    }
}

impl fmt::Display for DummyLang {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("dummy")
    }
}
