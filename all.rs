use crate::{
    BoundedQuantification, Exceptions, Existential, FOmega, FOmegaSub, LambdaOmega, Recursive,
    References, Stlc, Subtypes, SystemF, TypedArithmetic, UntypedArithmetic, UntypedLambda,
};
use errors::UndefinedLanguage;
use grammar::{LanguageDescribe, LanguageGrammar, LanguageRules};
use std::{fmt, str::FromStr};
use syntax::language::Language;

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
    /// Describe the language
    #[must_use]
    pub fn describe(&self) -> &str {
        match self {
            Self::UntypedArithmetic => UntypedArithmetic.describe(),
            Self::UntypedLambda => UntypedLambda.describe(),
            Self::TypedArithmetic => TypedArithmetic.describe(),
            Self::Stlc => Stlc.describe(),
            Self::Exceptions => Exceptions.describe(),
            Self::References => References.describe(),
            Self::Existential => Existential.describe(),
            Self::Recursive => Recursive.describe(),
            Self::Subtypes => Subtypes.describe(),
            Self::SystemF => SystemF.describe(),
            Self::BoundedQuantification => BoundedQuantification.describe(),
            Self::LambdaOmega => LambdaOmega.describe(),
            Self::FOmega => FOmega.describe(),
            Self::FOmegaSub => FOmegaSub.describe(),
        }
    }

    /// Get the language grammar
    #[must_use]
    pub fn grammars(&self) -> LanguageGrammar {
        match self {
            Self::UntypedArithmetic => UntypedArithmetic::grammars(),
            Self::UntypedLambda => UntypedLambda::grammars(),
            Self::TypedArithmetic => TypedArithmetic::grammars(),
            Self::Stlc => Stlc::grammars(),
            Self::Exceptions => Exceptions::grammars(),
            Self::References => References::grammars(),
            Self::Existential => Existential::grammars(),
            Self::Recursive => Recursive::grammars(),
            Self::Subtypes => Subtypes::grammars(),
            Self::SystemF => SystemF::grammars(),
            Self::BoundedQuantification => BoundedQuantification::grammars(),
            Self::LambdaOmega => LambdaOmega::grammars(),
            Self::FOmega => FOmega::grammars(),
            Self::FOmegaSub => FOmegaSub::grammars(),
        }
    }

    /// Get language rules of the language
    #[must_use]
    pub fn rules(&self) -> LanguageRules {
        match self {
            Self::UntypedArithmetic => UntypedArithmetic::rules(),
            Self::UntypedLambda => UntypedLambda::rules(),
            Self::TypedArithmetic => TypedArithmetic::rules(),
            Self::Stlc => Stlc::rules(),
            Self::Exceptions => Exceptions::rules(),
            Self::References => References::rules(),
            Self::Existential => Existential::rules(),
            Self::Recursive => Recursive::rules(),
            Self::Subtypes => Subtypes::rules(),
            Self::SystemF => SystemF::rules(),
            Self::BoundedQuantification => BoundedQuantification::rules(),
            Self::LambdaOmega => LambdaOmega::rules(),
            Self::FOmega => FOmega::rules(),
            Self::FOmegaSub => FOmegaSub::rules(),
        }
    }

    /// Is the language typed
    #[must_use]
    pub const fn is_typed(&self) -> bool {
        !matches!(self, Self::UntypedArithmetic | Self::UntypedLambda)
    }

    /// Get the language name
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::UntypedArithmetic => "UntypedArithmetic",
            Self::UntypedLambda => "UntypedLambda",
            Self::TypedArithmetic => "TypedArithmetic",
            Self::Stlc => "Stlc",
            Self::References => "References",
            Self::Exceptions => "Exceptions",
            Self::Subtypes => "Subtypes",
            Self::Recursive => "Recursive",
            Self::Existential => "Existential",
            Self::SystemF => "SystemF",
            Self::BoundedQuantification => "BoundedQuantification",
            Self::LambdaOmega => "LambdaOmega",
            Self::FOmega => "FOmega",
            Self::FOmegaSub => "FOmegaSub",
        }
    }
}

impl FromStr for AllLanguages {
    type Err = UndefinedLanguage;
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
            _ => Err(UndefinedLanguage::new(s)),
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
