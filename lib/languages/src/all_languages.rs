use crate::{
    BoundedQuantification, Exceptions, Existential, FOmega, FOmegaSub, LambdaOmega, Recursive,
    References, Stlc, Subtypes, SystemF, TypedArithmetic, UntypedArithmetic, UntypedLambda,
};
use errors::UndefinedLanguage;
use grammar::{LanguageDescribe, LanguageGrammar};
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
    pub fn describe(&self) -> &str {
        match self {
            AllLanguages::UntypedArithmetic => UntypedArithmetic.describe(),
            AllLanguages::UntypedLambda => UntypedLambda.describe(),
            AllLanguages::TypedArithmetic => TypedArithmetic.describe(),
            AllLanguages::Stlc => Stlc.describe(),
            AllLanguages::Exceptions => Exceptions.describe(),
            AllLanguages::References => References.describe(),
            AllLanguages::Existential => Existential.describe(),
            AllLanguages::Recursive => Recursive.describe(),
            AllLanguages::Subtypes => Subtypes.describe(),
            AllLanguages::SystemF => SystemF.describe(),
            AllLanguages::BoundedQuantification => BoundedQuantification.describe(),
            AllLanguages::LambdaOmega => LambdaOmega.describe(),
            AllLanguages::FOmega => FOmega.describe(),
            AllLanguages::FOmegaSub => FOmegaSub.describe(),
        }
    }

    pub fn grammars(&self) -> LanguageGrammar {
        match self {
            AllLanguages::UntypedArithmetic => UntypedArithmetic::grammars(),
            AllLanguages::UntypedLambda => UntypedLambda::grammars(),
            AllLanguages::TypedArithmetic => TypedArithmetic::grammars(),
            AllLanguages::Stlc => Stlc::grammars(),
            AllLanguages::Exceptions => Exceptions::grammars(),
            AllLanguages::References => References::grammars(),
            AllLanguages::Existential => Existential::grammars(),
            AllLanguages::Recursive => Recursive::grammars(),
            AllLanguages::Subtypes => Subtypes::grammars(),
            AllLanguages::SystemF => SystemF::grammars(),
            AllLanguages::BoundedQuantification => BoundedQuantification::grammars(),
            AllLanguages::LambdaOmega => LambdaOmega::grammars(),
            AllLanguages::FOmega => FOmega::grammars(),
            AllLanguages::FOmegaSub => FOmegaSub::grammars(),
        }
    }

    pub fn is_typed(&self) -> bool {
        !matches!(
            self,
            AllLanguages::UntypedArithmetic | AllLanguages::UntypedLambda
        )
    }

    pub fn name(&self) -> &'static str {
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
            _ => Err(UndefinedLanguage::new(s).into()),
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
