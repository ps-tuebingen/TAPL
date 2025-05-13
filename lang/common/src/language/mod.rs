use crate::{
    check::{CheckEnvironment, Subtypecheck, Typecheck},
    errors::{Error, ErrorKind, ErrorLocation},
    eval::{Eval, EvalEnvironment, Normalize},
};
use std::{fmt, str::FromStr};

mod terms;
mod types;
pub mod untyped;
mod values;

pub use terms::LanguageTerm;
pub use types::LanguageType;
pub use values::LanguageValue;

pub trait Language {
    type Term: LanguageTerm<Type = Self::Type, Value = Self::Value>
        + Typecheck<Env = Self::CheckEnv, Type = Self::Type>
        + Eval<Value = Self::Value, Env = Self::EvalEnv>;
    type Type: LanguageType
        + Subtypecheck<Self::Type, Env = Self::CheckEnv>
        + Normalize<Self::Type, Env = Self::CheckEnv>;
    type Value: LanguageValue<Term = Self::Term>;

    type CheckEnv: CheckEnvironment<Type = Self::Type>;
    type EvalEnv: EvalEnvironment<Self::Value>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImplementedLanguage {
    UntypedArithmetic,
    UntypedLambda,
    TypedArithmetic,
    Stlc,
    References,
    Exceptions,
    Subtypes,
    Recursive,
    Existential,
    SystemF,
    BoundedQuantification,
    LambdaOmega,
    FOmega,
    FOmegaSub,
}

impl ImplementedLanguage {
    pub const fn all() -> [ImplementedLanguage; 14] {
        [
            Self::UntypedArithmetic,
            Self::UntypedLambda,
            Self::TypedArithmetic,
            Self::Stlc,
            Self::References,
            Self::Exceptions,
            Self::Subtypes,
            Self::Recursive,
            Self::Existential,
            Self::SystemF,
            Self::BoundedQuantification,
            Self::LambdaOmega,
            Self::FOmega,
            Self::FOmegaSub,
        ]
    }

    pub fn describe(&self) -> &str {
        match self {
            Self::UntypedArithmetic => "Untyped Arithmetic Expressions",
            Self::UntypedLambda => "Untyped Lambda Calculus",
            Self::TypedArithmetic => "Typed Arithmetic Expressions",
            Self::Stlc => "Simply-Typed Lambda Calculus",
            Self::References => "STLC with References",
            Self::Exceptions => "STLC with Exceptions",
            Self::Subtypes => "STLC with Subtypes",
            Self::Recursive => "STLC with Recursive Types",
            Self::Existential => "STLC with Existential Types",
            Self::SystemF => "System-F",
            Self::BoundedQuantification => "System-F with bounded quantification",
            Self::LambdaOmega => "STLC with Higher-Kinded Types",
            Self::FOmega => "System-F with Higher-Kinded Types",
            Self::FOmegaSub => "System-F with Higher-Kinded Types and Subtyping",
        }
    }
}

impl FromStr for ImplementedLanguage {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "untyped-arithmetic" => Ok(Self::UntypedArithmetic),
            "untyped-lambda" => Ok(Self::UntypedLambda),
            "typed-arithmetic" => Ok(Self::TypedArithmetic),
            "stlc" => Ok(Self::Stlc),
            "references" => Ok(Self::References),
            "exceptions" => Ok(Self::Exceptions),
            "subtypes" => Ok(Self::Subtypes),
            "recursive" => Ok(Self::Recursive),
            "existential" => Ok(Self::Existential),
            "system-f" => Ok(Self::SystemF),
            "bounded-quantification" => Ok(Self::BoundedQuantification),
            "lambda-omega" => Ok(Self::LambdaOmega),
            "f-omega" => Ok(Self::FOmega),
            "f-omega-sub" => Ok(Self::FOmegaSub),
            _ => Err(Error {
                kind: ErrorKind::UndefinedLanguage(s.to_owned()),
                loc: ErrorLocation::LanguageSelect,
            }),
        }
    }
}

impl fmt::Display for ImplementedLanguage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::UntypedArithmetic => f.write_str("untyped-arithmetic"),
            Self::UntypedLambda => f.write_str("untyped-lambda"),
            Self::TypedArithmetic => f.write_str("typed-arithmetic"),
            Self::Stlc => f.write_str("stlc"),
            Self::References => f.write_str("references"),
            Self::Exceptions => f.write_str("exceptions"),
            Self::Subtypes => f.write_str("subtypes"),
            Self::Recursive => f.write_str("recursive"),
            Self::Existential => f.write_str("existential"),
            Self::SystemF => f.write_str("system-f"),
            Self::BoundedQuantification => f.write_str("bounded-quantification"),
            Self::LambdaOmega => f.write_str("lambda-omega"),
            Self::FOmega => f.write_str("f-omega"),
            Self::FOmegaSub => f.write_str("f-omega-sub"),
        }
    }
}

#[cfg(test)]
mod lang_tests {
    use super::ImplementedLanguage;
    use std::str::FromStr;

    #[test]
    fn from_str_to_str() {
        for lang in ImplementedLanguage::all() {
            let from_to = ImplementedLanguage::from_str(&lang.to_string()).unwrap();
            assert_eq!(lang, from_to)
        }
    }
}
