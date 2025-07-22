use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllLanguages {
    UntypedArithmetic,
    UntypedLambda,
    TypedArithmetic,
    Stlc,
    Exceptions,
    References,
    Existential,
    Recursive,
    Subtypes,
    SystemF,
    BoundedQuantification,
    LambdaOmega,
    FOmega,
    FOmegaSub,
}

impl AllLanguages {
    pub fn all() -> [AllLanguages; 14] {
        [
            AllLanguages::UntypedArithmetic,
            AllLanguages::UntypedLambda,
            AllLanguages::TypedArithmetic,
            AllLanguages::Stlc,
            AllLanguages::Exceptions,
            AllLanguages::References,
            AllLanguages::Existential,
            AllLanguages::Recursive,
            AllLanguages::Subtypes,
            AllLanguages::SystemF,
            AllLanguages::BoundedQuantification,
            AllLanguages::LambdaOmega,
            AllLanguages::FOmega,
            AllLanguages::FOmegaSub,
        ]
    }
}

impl FromStr for AllLanguages {
    type Err = LanguageError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "untyped-arithmetic" => Ok(AllLanguages::UntypedArithmetic),
            "untyped-lambda" => Ok(AllLanguages::UntypedLambda),
            "typed-arithmetic" => Ok(AllLanguages::TypedArithmetic),
            "stlc" => Ok(AllLanguages::Stlc),
            "references" => Ok(AllLanguages::References),
            "exceptions" => Ok(AllLanguages::Exceptions),
            "subtypes" => Ok(AllLanguages::Subtypes),
            "recursive" => Ok(AllLanguages::Recursive),
            "existential" => Ok(AllLanguages::Existential),
            "system-f" => Ok(AllLanguages::SystemF),
            "bounded-quantification" => Ok(AllLanguages::BoundedQuantification),
            "lambda-omega" => Ok(AllLanguages::LambdaOmega),
            "f-omega" => Ok(AllLanguages::FOmega),
            "f-omega-sub" => Ok(AllLanguages::FOmegaSub),
            _ => Err(LanguageError::UndefinedLanguage(s.to_owned())),
        }
    }
}

impl fmt::Display for AllLanguages {
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
